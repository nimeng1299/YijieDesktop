use dashmap::DashMap;
use lazy_static::lazy_static;

use crate::{player, tauris::{base, tabs::TABS}};

lazy_static!{
    pub static ref PLAYER_MAP: DashMap<u32, player::PlayerSocket> = DashMap::new();
}
//必须在启动后就调用
#[tauri::command]
pub fn init_app(app: tauri::AppHandle){
    if !base::TAURI_INIT.load(std::sync::atomic::Ordering::Relaxed){
        base::APP.set(app).unwrap();
        
        base::TAURI_INIT.store(true, std::sync::atomic::Ordering::Relaxed);
    }
    
}

//添加Tab
#[tauri::command]
pub fn add_tab_main(){
    TABS.lock().unwrap().add_main();
}

//删除Tab
#[tauri::command]
pub fn close_tab(tab_id: u32){
    TABS.lock().unwrap().close(tab_id);
}

#[tauri::command]
pub fn login(app: tauri::AppHandle, tab_id: u32, ip: &str, name: &str) -> Result<(), String> {
    if PLAYER_MAP.contains_key(&tab_id) {
        return Err("player already exists".to_string());
    }
    let mut player_socket = player::PlayerSocket::connect(app, tab_id, ip).map_err(|e| e.to_string())?;
    
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

#[tauri::command]
pub fn close(tab_id: u32) -> Result<(), String> {
    if !PLAYER_MAP.contains_key(&tab_id) {
        return Err("player not exists".to_string());
    }
    let (_, player_socket) = PLAYER_MAP.remove(&tab_id).unwrap();
    player_socket.close();
    Ok(())
}

