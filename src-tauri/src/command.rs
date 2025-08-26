use dashmap::DashMap;
use lazy_static::lazy_static;

use crate::player;

lazy_static!{
    pub static ref PLAYER_MAP: DashMap<u32, player::PlayerSocket> = DashMap::new();
}


#[tauri::command]
pub fn login(tab_id: u32, ip: &str, name: &str) -> Result<(), String> {
    let mut player_socket = player::PlayerSocket::connect(ip).map_err(|e| e.to_string())?;
    
    // 等待连接建立
    let mut connected = false;
    for i in 0..5 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let player = player_socket.get_player().map_err(|e| e.to_string())?;
        if player.isConnected {
            connected = true;
            break;
        }
        println!("wait for connection {} / 5", i);
    }
    
    if !connected {
        return Err("connection failed".to_string());
    }
    
    let mut login_success = false;
    for i in 0..5{
        let player = player_socket.get_player().map_err(|e| e.to_string())?;
        let res = player.login(name);
        match res {
            Ok(_) => {
                login_success = true;
                break;
            }
            Err(e) => {
                println!("login error: {}, try again {} / 5", e, i);
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
    if !login_success{
        return Err("login failed".to_string());
    }
    PLAYER_MAP.insert(tab_id, player_socket);
    Ok(())
}