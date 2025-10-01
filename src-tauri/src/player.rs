use std::{
    collections::VecDeque,
    io::{Read, Write},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{self, Duration},
};

use crate::{
    account::Account,
    content::{game::Game, hall_room_list::HallRoomList, room::Room},
    listen,
    socket::msger::{self, Msger},
};
use anyhow::{anyhow, bail, Result};
use java_string::JavaString;
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{mpsc, Mutex, MutexGuard},
    task::JoinHandle,
};

pub struct PlayerSocket {
    pub player: Arc<Mutex<Player>>,
    tasks: Vec<JoinHandle<()>>,
}

impl PlayerSocket {
    pub async fn connect(app: tauri::AppHandle, ip: &str) -> Result<Self> {
        let mut stream = TcpStream::connect(ip).await?;
        let (mut read_stream, mut write_stream) = stream.into_split();

        let (send_tx, mut send_rx) = mpsc::unbounded_channel::<Vec<u8>>();
        let tx1 = send_tx.clone();

        let player = Arc::new(Mutex::new(Player::new(send_tx, app)));
        let player_read = Arc::clone(&player);
        let player_keeplive = Arc::clone(&player);

        let read_task = tokio::spawn(async move {
            let mut buffer = [0; 2048]; // Temporary buffer for reading from socket
            let mut data_buffer: VecDeque<u8> = VecDeque::new();
            loop {
                match read_stream.read(&mut buffer).await {
                    Ok(0) => {
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
                            tokio::spawn(async move {
                                if let Ok(java_str) = JavaString::from_modified_utf8(msg_bytes) {
                                    let mut player = player_clone.lock().await;
                                    if let Err(e) = player.read(java_str.trim().to_string()).await {
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
                        break;
                    }
                }
            }
        });

        let write_task = tokio::spawn(async move {
            'outer: loop {
                while let Some(msg) = send_rx.recv().await {
                    match write_stream.write_all(&msg).await {
                        Ok(_) => {}
                        Err(e) => {
                            listen::show_toast(e.to_string().as_str(), "error");
                            break 'outer;
                        }
                    }

                    println!("write: {:?}", msg);
                }
            }
        });

        let keeplive_task = tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(25)).await;
                let msg = Msger::KeepLive.to_msg("Ok".to_string());
                tx1.send(msg);
                let now = time::SystemTime::now();
                let epoch = now
                    .duration_since(time::SystemTime::UNIX_EPOCH)
                    .unwrap_or_default();
                let seconds = epoch.as_secs() % 86400; // 当天秒数
                let hour = (seconds / 3600) % 24;
                let minute = (seconds % 3600) / 60;
                let second = seconds % 60;
                println!("keeplive: {:02}:{:02}:{:02}", hour, minute, second);
            }
        });

        Ok(Self {
            player,
            tasks: vec![read_task, write_task, keeplive_task],
        })
    }

    pub async fn send(&self, msg: Vec<u8>) -> Result<()> {
        self.player.lock().await.send(msg).await
    }

    pub async fn get_player(&self) -> MutexGuard<Player> {
        self.player.lock().await
    }

    pub async fn close(&self) -> Result<()> {
        for task in &self.tasks {
            task.abort();
        }
        Ok(())
    }
}

impl Drop for PlayerSocket {
    fn drop(&mut self) {
        self.close();
    }
}

#[derive(Debug)]
pub struct Player {
    send_tx: mpsc::UnboundedSender<Vec<u8>>,
    pub isConnected: bool,

    app: tauri::AppHandle,
    data: Data,

    pub isLogin: bool,
}

impl Player {
    pub fn new(send_tx: mpsc::UnboundedSender<Vec<u8>>, app: tauri::AppHandle) -> Self {
        Player {
            send_tx,
            isConnected: false,
            app,
            data: Data::default(),
            isLogin: false,
        }
    }

    pub async fn send(&self, msg: Vec<u8>) -> Result<()> {
        self.send_tx.send(msg)?;
        Ok(())
    }

    pub fn send_no_async(&self, msg: Vec<u8>) -> Result<()> {
        self.send_tx.send(msg)?;
        Ok(())
    }

    pub async fn read(&mut self, msgs: String) -> Result<()> {
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
                listen::show_toast("登录成功", "success");
                self.isLogin = true;
            }
            Msger::RefreshRoomList => {
                let room_list = HallRoomList::from_string(msg);
                println!("refresh room list {}", room_list.rooms.len());
                self.data.change_to_hall(self.app.clone(), room_list);
            }
            Msger::RefreshPlayerInfo => {
                let account = Account::from_msg(msg)?;
                self.data.change_account(self.app.clone(), account);
            }
            Msger::RefreshRoomInfo => match Room::from_msg(msg) {
                Ok(room) => {
                    self.data.change_to_room(self.app.clone(), room.clone());
                }
                Err(e) => {
                    println!("change_to_room error: {}", e);
                }
            },
            Msger::RefreshGameInfo => {
                match Game::from_msg(msg, |m| {
                    let _ = self.send_no_async(Msger::RequestCacheSignContent.to_msg(m));
                }) {
                    Ok(game) => {
                        self.data.update_game(self.app.clone(), game);
                    }
                    Err(e) => {
                        println!("update_game error: {}", e);
                    }
                }
            }
            Msger::DispatchCustomBottom => {
                let mut buttons: Vec<String> = msg.split(';').map(|s| s.to_string()).collect();
                if buttons.len() == 1 && buttons[0] == "-1" {
                    buttons = Vec::new();
                }
                self.data.dispatch_custom_bottom(self.app.clone(), buttons);
            }
            Msger::RefreshCountdown => {
                let countdown: Vec<&str> = msg.split('&').collect();
                let countdown = (countdown[0].parse()?, countdown[1].parse()?);
                self.data.refresh_countdown(self.app.clone(), countdown);
            }
            Msger::YouCanMove => {
                self.data.change_move(self.app.clone(), true);
            }
            Msger::YouNotMove => {
                self.data.change_move(self.app.clone(), false);
            }
            Msger::ReturnCacheSignContent => {
                self.data.game.add_cache_sign(msg);
                listen::update_game(self.app.clone(), self.data.game.clone());
            }
            Msger::LoginFailed => {
                listen::show_toast(format!("登录失败: {msg}").as_str(), "error");
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

    pub async fn login(&self, name: &str) -> Result<()> {
        if self.isConnected {
            let msg = Msger::RequestLogin.to_msg(format!("{}@v1.4.0", name));
            self.send(msg).await;
            Ok(())
        } else {
            Err(anyhow!("player not connected"))
        }
    }

    pub async fn request_enter_room(&self, room_name: String) -> Result<()> {
        if self.isLogin {
            let msg = Msger::RequestEnterRoom.to_msg(room_name);
            self.send(msg).await;
            Ok(())
        } else {
            bail!("need login!")
        }
    }

    pub fn get_data(&self) -> Data {
        self.data.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Data {
    mode: String,
    roomdata: HallRoomList,
    account: Account,
    room: Room,
    game: Game,
    countdown: (i32, i32),
    buttons: Vec<String>,
    can_move: bool,
}

impl Data {
    pub fn change_mode(&mut self, app: tauri::AppHandle, mode: Modes) {
        self.mode = mode.to_string();
        listen::change_mode(app, mode.to_string());
    }

    pub fn change_to_hall(&mut self, app: tauri::AppHandle, room_list: HallRoomList) {
        self.roomdata = room_list.clone();
        self.change_mode(app.clone(), Modes::RoomList);
        listen::change_to_hall(app, room_list);
    }

    pub fn change_account(&mut self, app: tauri::AppHandle, account: Account) {
        self.account = account.clone();
        listen::change_account(app, account);
    }

    pub fn change_to_room(&mut self, app: tauri::AppHandle, room: Room) {
        self.room = room.clone();
        self.change_mode(app.clone(), Modes::Game);
        listen::change_to_room(app, room);
    }

    pub fn update_game(&mut self, app: tauri::AppHandle, game: Game) {
        self.game = game.clone();
        listen::update_game(app, game);
    }

    pub fn dispatch_custom_bottom(&mut self, app: tauri::AppHandle, buttons: Vec<String>) {
        self.buttons = buttons.clone();
        listen::dispatch_custom_bottom(app, buttons);
    }

    pub fn refresh_countdown(&mut self, app: tauri::AppHandle, countdown: (i32, i32)) {
        self.countdown = countdown;
        listen::refresh_countdown(app, countdown);
    }

    pub fn change_move(&mut self, app: tauri::AppHandle, can_move: bool) {
        self.can_move = can_move;
        if can_move {
            listen::you_can_move(app);
        } else {
            listen::you_not_move(app);
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            mode: Modes::default().to_string(),
            roomdata: HallRoomList::default(),
            account: Account::default(),
            room: Room::default(),
            game: Game::default(),
            countdown: (0, 0),
            buttons: vec![],
            can_move: false,
        }
    }
}

//界面 mode
#[derive(Debug)]
pub enum Modes {
    Login,
    RoomList,
    Game,
}

impl Modes {
    fn to_string(&self) -> String {
        let str = match self {
            Self::Login => "login",
            Self::RoomList => "roomlist",
            Self::Game => "game",
        };
        str.to_string()
    }
}

impl Default for Modes {
    fn default() -> Self {
        Self::Login
    }
}
