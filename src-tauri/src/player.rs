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
    tauris::{change_tab_title, do_tab_datas},
};
use anyhow::{anyhow, bail, Result};
use java_string::JavaString;
use std::net::TcpStream;

pub struct PlayerSocket {
    pub player: Arc<Mutex<Player>>,
    running: Arc<AtomicBool>,
}

impl PlayerSocket {
    pub async fn connect(app: tauri::AppHandle, tab_id: u32, ip: &str) -> Result<Self> {
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
            let mut buffer = [0; 2048]; // Temporary buffer for reading from socket
            let mut can_start = false;
            let mut data_len = 0;
            let mut data_buffer: VecDeque<u8> = VecDeque::new();
            while running_read.load(Ordering::Relaxed) {
                match read_stream.read(&mut buffer) {
                    Ok(0) => {
                        running_read.store(false, Ordering::Relaxed);
                        break;
                    }
                    Ok(n) => {
                        println!("read: {} bytes", n);
                        data_buffer.extend(&buffer[0..n]);

                        // 循环处理所有完整数据包
                        while data_buffer.len() >= 2 {
                            let data_len =
                                u16::from_be_bytes([data_buffer[0], data_buffer[1]]) as usize;

                            if data_buffer.len() < data_len + 2 {
                                break; // 等待更多数据
                            }

                            // 移除长度头
                            data_buffer.drain(0..2);
                            let msg_bytes: Vec<u8> = data_buffer.drain(0..data_len).collect();

                            // 异步处理避免阻塞读取线程
                            let player_clone = Arc::clone(&player_read);
                            thread::spawn(move || {
                                if let Ok(java_str) = JavaString::from_modified_utf8(msg_bytes) {
                                    let mut player = player_clone.lock().unwrap();
                                    if let Err(e) = player.read(java_str.trim().to_string()) {
                                        println!("处理错误: {}", e);
                                    }
                                }
                            });
                        }
                    }

                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(1));
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
                    match write_stream.write_all(&msg) {
                        Ok(_) => {}
                        Err(e) => {
                            running_write.store(false, Ordering::Relaxed);
                            listen::show_toast(e.to_string().as_str(), "error");
                        }
                    }

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
}

impl Player {
    pub fn new(send_tx: mpsc::Sender<Vec<u8>>, tab_id: u32, app: tauri::AppHandle) -> Self {
        Player {
            send_tx,
            isConnected: false,
            tab_id,
            app,
            isLogin: false,
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
                change_tab_title(self.app.clone(), self.tab_id, msg);
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
                    do_tab_datas(self.tab_id, |data| {
                        data.change_to_room(self.app.clone(), self.tab_id, room.clone());
                    });
                }
                Err(e) => {
                    println!("change_to_room error: {}", e);
                }
            },
            Msger::RefreshGameInfo => match Game::from_msg(msg) {
                Ok(game) => {
                    do_tab_datas(self.tab_id, |data| {
                        data.update_game(self.app.clone(), self.tab_id, game);
                    });
                }
                Err(e) => {
                    println!("update_game error: {}", e);
                }
            },
            Msger::DispatchCustomBottom => {
                let mut buttons: Vec<String> = msg.split(';').map(|s| s.to_string()).collect();
                if buttons.len() == 1 && buttons[0] == "-1" {
                    buttons = Vec::new();
                }
                do_tab_datas(self.tab_id, |data| {
                    data.dispatch_custom_bottom(self.app.clone(), self.tab_id, buttons);
                })
            }
            Msger::RefreshCountdown => {
                let countdown: Vec<&str> = msg.split('&').collect();
                let countdown = (countdown[0].parse()?, countdown[1].parse()?);
                do_tab_datas(self.tab_id, |data| {
                    data.refresh_countdown(self.app.clone(), self.tab_id, countdown);
                });
            }
            Msger::YouCanMove => {
                do_tab_datas(self.tab_id, |data| {
                    data.change_move(self.app.clone(), self.tab_id, true);
                });
            }
            Msger::YouNotMove => {
                do_tab_datas(self.tab_id, |data| {
                    data.change_move(self.app.clone(), self.tab_id, false);
                });
            }
            Msger::WinMessage | Msger::GameStart => {
                listen::show_toast(msg.as_str(), "success");
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
