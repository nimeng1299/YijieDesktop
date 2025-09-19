use std::{
    collections::VecDeque,
    io::{Read, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex, MutexGuard,
    },
    thread,
    time::Duration,
};

use crate::{
    account::Account,
    content::{game::Game, hall_room_list::HallRoomList, room::Room},
    listen,
    socket::msger::{self, Msger},
    tauris::do_tab_datas,
};
use anyhow::{anyhow, bail, Result};
use java_string::JavaString;
use std::net::TcpStream;

pub struct PlayerSocket {
    pub player: Arc<Mutex<Player>>,
    running: Arc<AtomicBool>,
}

impl PlayerSocket {
    pub fn connect(app: tauri::AppHandle, tab_id: u32, ip: &str) -> Result<Self> {
        let mut read_stream = TcpStream::connect(ip)?;
        let mut write_stream = read_stream.try_clone()?;

        let running = Arc::new(AtomicBool::new(true));
        let running_read = Arc::clone(&running);
        let running_write = Arc::clone(&running);
        let running_keeplive = Arc::clone(&running);

        let (send_tx, send_rx) = mpsc::channel::<Vec<u8>>();

        let player = Arc::new(Mutex::new(Player::new(send_tx, tab_id, app)));
        let player_read = Arc::clone(&player);
        let player_keeplive = Arc::clone(&player);

        let read_thread = thread::spawn(move || {
            let mut buffer = [0; 1024]; // Temporary buffer for reading from socket
            let mut can_start = false;
            let mut data_len = 0;
            let mut datas: VecDeque<u8> = VecDeque::new();
            while running_read.load(Ordering::Relaxed) {
                match read_stream.read(&mut buffer) {
                    Ok(0) => {
                        running_read.store(false, Ordering::Relaxed);
                        break;
                    }
                    Ok(n) => {
                        println!("read: {} bytes", n);
                        datas.extend(&buffer[0..n]);

                        if !can_start && datas.len() > 2 {
                            let len_datas = datas.drain(..2).collect::<Vec<u8>>();
                            data_len = u16::from_be_bytes([len_datas[0], len_datas[1]]) as usize;
                            println!("data_len: {}. let_datas: {:?}", data_len, len_datas);
                            can_start = true;
                        }
                        if can_start && datas.len() >= data_len {
                            let msg_datas = datas.drain(..data_len).collect::<Vec<u8>>();
                            match JavaString::from_modified_utf8(msg_datas) {
                                Ok(java_str) => {
                                    println!("java_str: {}", java_str);
                                    let mut player = player_read.lock().unwrap();
                                    match player.read(java_str.trim().to_string()) {
                                        Ok(_) => {}
                                        Err(e) => {
                                            println!("read error: {}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("from_modified_utf8 error: {}", e);
                                }
                            }
                            can_start = false;
                        }
                    }

                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(100));
                        continue;
                    }
                    Err(e) => {
                        println!("{}", e);
                        running_read.store(false, Ordering::Relaxed);
                    }
                }
            }
        });

        let write_thread = thread::spawn(move || {
            while running_write.load(Ordering::Relaxed) {
                while let Ok(msg) = send_rx.recv() {
                    write_stream.write_all(&msg).unwrap();
                    println!("write: {:?}", msg);
                }
            }
        });

        let keeplive_thread = thread::spawn(move || {
            while running_keeplive.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_secs(30));
                let msg = Msger::KeepLive.to_msg("Ok".to_string());
                player_keeplive.lock().unwrap().send(msg);
                println!("keeplive");
            }
        });

        Ok(Self { player, running })
    }

    pub fn send(&self, msg: Vec<u8>) -> Result<()> {
        self.player.lock().unwrap().send(msg)
    }

    pub fn get_player(&self) -> Result<MutexGuard<Player>> {
        Ok(self
            .player
            .lock()
            .map_err(|e| anyhow!("Failed to acquire player lock: {}", e))?)
    }

    pub fn close(&self) -> Result<()> {
        self.running.store(false, Ordering::Relaxed);
        Ok(())
    }
}

impl Drop for PlayerSocket {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

#[derive(Debug)]
pub struct Player {
    send_tx: mpsc::Sender<Vec<u8>>,
    pub isConnected: bool,

    app: tauri::AppHandle,
    tab_id: u32,

    pub isLogin: bool,

    pub room: Option<Room>,
    pub game: Option<Game>,
}

impl Player {
    pub fn new(send_tx: mpsc::Sender<Vec<u8>>, tab_id: u32, app: tauri::AppHandle) -> Self {
        Player {
            send_tx,
            isConnected: false,
            tab_id,
            app,
            isLogin: false,
            room: None,
            game: None,
        }
    }

    pub fn send(&self, msg: Vec<u8>) -> Result<()> {
        self.send_tx.send(msg)?;
        Ok(())
    }

    pub fn read(&mut self, msgs: String) -> Result<()> {
        let (msg_type, msg) = Msger::parse(msgs)?;
        match msg_type {
            Msger::ConnSuccess => {
                if msg == "Ok" {
                    println!("conn success");
                    self.isConnected = true;
                }
            }
            Msger::LoginSuccess => {
                println!("login success");
                self.isLogin = true;
                listen::login_success(self.app.clone(), self.tab_id, msg);
            }
            Msger::RefreshRoomList => {
                let room_list = HallRoomList::from_string(msg);
                println!("refresh room list {}", room_list.rooms.len());
                do_tab_datas(self.tab_id, |data| {
                    data.change_to_hall(self.app.clone(), self.tab_id, room_list);
                });
            }
            Msger::RefreshPlayerInfo => {
                let account = Account::from_msg(msg)?;
                do_tab_datas(self.tab_id, |data| {
                    data.change_account(self.app.clone(), self.tab_id, account);
                });
            }
            Msger::RefreshRoomInfo => match Room::from_msg(msg) {
                Ok(room) => {
                    listen::change_to_room(self.app.clone(), self.tab_id, room.clone());
                    self.room = Some(room);
                }
                Err(e) => {
                    println!("change_to_room error: {}", e);
                }
            },
            Msger::RefreshGameInfo => match Game::from_msg(msg) {
                Ok(game) => {
                    listen::update_game(self.app.clone(), self.tab_id, game.clone());
                    self.game = Some(game);
                }
                Err(e) => {
                    println!("update_game error: {}", e);
                }
            },
            Msger::DispatchCustomBottom => {
                let buttons: Vec<String> = msg.split(';').map(|s| s.to_string()).collect();
                listen::dispatch_custom_bottom(self.app.clone(), self.tab_id, buttons);
            }
            Msger::RefreshCountdown => {
                let countdown: Vec<&str> = msg.split('&').collect();
                let countdown = (countdown[0].parse()?, countdown[1].parse()?);
                listen::refresh_countdown(self.app.clone(), self.tab_id, countdown);
            }
            Msger::YouCanMove => {
                listen::you_can_move(self.app.clone(), self.tab_id);
            }
            Msger::YouNotMove => {
                listen::you_not_move(self.app.clone(), self.tab_id);
            }
            _ => {
                println!("--> read: {} type: {}", msg, msg_type);
            }
        }
        Ok(())
    }

    pub fn login(&self, name: &str) -> Result<()> {
        if self.isConnected {
            let msg = Msger::RequestLogin.to_msg(format!("{}@v1.4.0", name));
            self.send(msg)?;
            Ok(())
        } else {
            Err(anyhow!("player not connected"))
        }
    }

    pub fn request_enter_room(&self, room_name: String) -> Result<()> {
        if self.isLogin {
            let msg = Msger::RequestEnterRoom.to_msg(room_name);
            self.send(msg)?;
            Ok(())
        } else {
            bail!("need login!")
        }
    }
}
