use std::{
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
};
use anyhow::{anyhow, bail, Result};
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
            let mut buffer = [0; 512];
            while running_read.load(Ordering::Relaxed) {
                match read_stream.read(&mut buffer) {
                    Ok(0) => {
                        running_read.store(false, Ordering::Relaxed);
                        break;
                    }
                    Ok(n) => {
                        let mut start = 0;
                        let mut end = n;
                        println!("read: {:?}", &buffer[0..n]);

                        match msger::read_utf(&mut &buffer[0..n]) {
                            Ok(msg) => {
                                for s in msg {
                                    println!("msg: {}", s);

                                    let mut player = player_read.lock().unwrap();
                                    match player.read(s.trim().to_string()) {
                                        Ok(_) => {}
                                        Err(e) => {
                                            println!("read error: {}", e);
                                        }
                                    }
                                }
                            }

                            Err(e) => {
                                println!("read_utf error: {}", e);
                            }
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
    state: u8, //状态： 0登录界面 1大厅 2房间

    pub account: Account,
    pub isLogin: bool,

    pub hall_room_list: HallRoomList,
    pub room: Option<Room>,
    pub game: Option<Game>,
}

impl Player {
    pub fn new(send_tx: mpsc::Sender<Vec<u8>>, tab_id: u32, app: tauri::AppHandle) -> Self {
        Player {
            send_tx,
            isConnected: false,
            tab_id,
            state: 0,
            app,
            account: Account::default(),
            isLogin: false,
            hall_room_list: HallRoomList::default(),
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
                self.hall_room_list = room_list;
                println!("refresh room list {}", self.hall_room_list.rooms.len());
                self.set_state(1);
            }
            Msger::RefreshPlayerInfo => {
                let account = Account::from_msg(msg)?;
                self.account = account;
                listen::change_account(self.app.clone(), self.tab_id, self.account.clone());
            }
            Msger::RefreshRoomInfo => {
              match Room::from_msg(msg){
                    Ok(room) => {
                        listen::change_to_room(self.app.clone(), self.tab_id, room.clone());
                        self.room = Some(room);
                    }
                    Err(e) => {
                        println!("change_to_room error: {}", e);
                    }
                }
            }
            Msger::RefreshGameInfo => {
                match Game::from_msg(msg){
                    Ok(game) => {
                        listen::update_game(self.app.clone(), self.tab_id, game.clone());
                        self.game = Some(game);
                    }
                    Err(e) => {
                        println!("update_game error: {}", e);
                    }
                }
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

    pub fn set_state(&mut self, state: u8) {
        self.state = state;
        match state {
            1 => {
                listen::change_to_hall(self.app.clone(), self.tab_id, self.hall_room_list.clone());
            }
            _ => {}
        }
    }

    pub fn request_enter_room(&self, room_name:String) -> Result<()>{
        if self.isLogin{
            let msg = Msger::RequestEnterRoom.to_msg(room_name);
            self.send(msg)?;
            Ok(())
        }else{
            bail!("need login!")
        }
    }
}
