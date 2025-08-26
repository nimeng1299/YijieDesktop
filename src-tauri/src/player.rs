use std::{io::{Read, Write}, sync::{atomic::{AtomicBool, Ordering}, mpsc, Arc, Mutex, MutexGuard}, thread, time::Duration};

use crate::{account::Account, socket::msger::Msger};
use anyhow::{anyhow, bail, Result};
use std::net::TcpStream;

pub struct PlayerSocket {
    pub player: Arc<Mutex<Player>>,
}

impl PlayerSocket {
    pub fn connect(ip: &str) -> Result<Self> {
        let mut read_stream = TcpStream::connect(ip)?;
        let mut write_stream = read_stream.try_clone()?;

        let running = Arc::new(AtomicBool::new(true));
        let running_read = Arc::clone(&running);
        let running_write = Arc::clone(&running);

        let (send_tx, send_rx) = mpsc::channel::<Vec<u8>>();

        let player = Arc::new(Mutex::new(Player::new(send_tx)));
        let player_read = Arc::clone(&player);



        let read_thread = thread::spawn(move || {
             let mut buffer = [0; 512];
             while running_read.load(Ordering::Relaxed) {
                match read_stream.read(&mut buffer) {
                    Ok(0) => {
                        running_read.store(false, Ordering::Relaxed);
                        break;
                    }
                    Ok(n) => {
                        let msg = String::from_utf8_lossy(&buffer[2..n]);//舍弃前两个字符
                        println!("read: {}", msg);
                        let mut player = player_read.lock().unwrap();
                        match player.read(msg.trim().to_string()){
                            Ok(_) => {}
                            Err(e) => {
                                println!("read error: {}", e);
                            }
                        }
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock=> {
                        thread::sleep(Duration::from_millis(100));
                        continue;
                    }
                    Err(e) => {
                        println!("{}", e);
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

        Ok(Self {
            player,
        })
    }

    pub fn send(&self, msg: Vec<u8>) -> Result<()> {
        self.player.lock().unwrap().send(msg)
    }

    pub fn get_player(&self) -> Result<MutexGuard<Player>> {
        Ok(self.player.lock().map_err(|e| {
            anyhow!("Failed to acquire player lock: {}", e)
        })?)
    }
}

#[derive(Debug)]
pub struct Player {
    send_tx: mpsc::Sender<Vec<u8>>,
    pub isConnected: bool,

    pub account: Account,
    pub isLogin: bool,
}




impl Player {
    pub fn new(send_tx: mpsc::Sender<Vec<u8>>) -> Self{
        Player {
            send_tx,
            isConnected: false,
            account: Account::default(),
            isLogin: false,
        }
    }


    pub fn send(&self, msg: Vec<u8>) -> Result<()> {
        self.send_tx.send(msg)?;
        Ok(())
    }

    pub  fn read(& mut self, msg: String) -> Result<()> {
        let (msg_type, msg) = Msger::parse(msg)?;
        match  msg_type{
            Msger::ConnSuccess => {
                if msg == "Ok"{
                    println!("conn success");
                    self.isConnected = true;
                }
            }
            _ => {
                println!("read: {} type: {}", msg, msg_type);
            }
        }
        Ok(())
    }

    pub fn login(&self, name: &str) -> Result<()>{
        if self.isConnected{
            let msg = Msger::RequestLogin.to_msg(format!("{}@v1.4.0", name));
            self.send(msg)?;
            Ok(())
        }else{
            Err(anyhow!("player not connected"))
        }
    }
}
