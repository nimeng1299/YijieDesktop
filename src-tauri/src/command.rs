use dashmap::DashMap;
use lazy_static::lazy_static;

use crate::{
    listen::show_toast,
    player,
    socket::msger::Msger,
    tauris::{
        base, create_tab_datas, create_tab_title, delete_tab_title, get_tab_data, tab_data::TabData,
    },
};

lazy_static! {
    pub static ref PLAYER_MAP: DashMap<u32, player::PlayerSocket> = DashMap::new();
}
//必须在启动后就调用
#[tauri::command]
pub fn init_app(app: tauri::AppHandle) {
    if !base::TAURI_INIT.load(std::sync::atomic::Ordering::Relaxed) {
        base::APP.set(app).unwrap();
        //创建第一个窗口
        create_tab_title();

        base::TAURI_INIT.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

//添加Tab
#[tauri::command]
pub fn add_tab_main() {
    create_tab_title();
}

//删除Tab
#[tauri::command]
pub fn close_tab(tab_id: u32) {
    delete_tab_title(tab_id);
}

/// 刷新数据
#[tauri::command]
pub fn refresh_data(tab_id: u32) -> Result<TabData, String> {
    get_tab_data(tab_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn login(app: tauri::AppHandle, tab_id: u32, ip: &str, name: &str) -> Result<(), String> {
    let ip = if ip.is_empty() {
        "47.100.88.110:20003"
    } else {
        ip
    };
    if PLAYER_MAP.contains_key(&tab_id) {
        PLAYER_MAP.remove(&tab_id);
    }
    let mut player_socket = player::PlayerSocket::connect(app, tab_id, ip)
        .await
        .map_err(|e| e.to_string())?;

    // 等待连接建立
    let mut connected = false;
    for i in 0..5 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let player = player_socket.get_player().await;
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
    for i in 0..5 {
        let player = player_socket.get_player().await;
        let res = player.login(name).await;
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
    if !login_success {
        return Err("login failed".to_string());
    }
    PLAYER_MAP.insert(tab_id, player_socket);
    create_tab_datas(tab_id);
    show_toast("登录成功", "success");
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

#[tauri::command]
pub async fn request_enter_room(tab_id: u32, room_name: &str) -> Result<(), String> {
    if let Some(player_socket) = PLAYER_MAP.get(&tab_id) {
        player_socket
            .get_player()
            .await
            .request_enter_room(room_name.to_string())
            .await;
        Ok(())
    } else {
        Err("player not exists".to_string())
    }
}

#[tauri::command]
pub async fn request_be_chess_player(tab_id: u32, side: &str) -> Result<(), String> {
    if let Some(player_socket) = PLAYER_MAP.get(&tab_id) {
        player_socket
            .get_player()
            .await
            .send(Msger::RequestBeChessPlayer.to_msg(side.to_string()))
            .await;
        Ok(())
    } else {
        Err("player not exists".to_string())
    }
}

//让座
#[tauri::command]
pub async fn request_leave_seat(tab_id: u32) -> Result<(), String> {
    if let Some(player_socket) = PLAYER_MAP.get(&tab_id) {
        player_socket
            .get_player()
            .await
            .send(Msger::RequestLeaveSeat.to_msg("Ok".to_string()))
            .await;
        Ok(())
    } else {
        Err("player not exists".to_string())
    }
}

//认输
#[tauri::command]
pub async fn request_admit_defeat(tab_id: u32) -> Result<(), String> {
    if let Some(player_socket) = PLAYER_MAP.get(&tab_id) {
        player_socket
            .get_player()
            .await
            .send(Msger::RequestAdmitDefeat.to_msg("Ok".to_string()))
            .await;
        Ok(())
    } else {
        Err("player not exists".to_string())
    }
}

//按钮requestCustomBottomEvent
#[tauri::command]
pub async fn request_custom_bottom_event(tab_id: u32, event: &str) -> Result<(), String> {
    if let Some(player_socket) = PLAYER_MAP.get(&tab_id) {
        player_socket
            .get_player()
            .await
            .send(Msger::RequestCustomBottomEvent.to_msg(event.to_string()))
            .await;
        Ok(())
    } else {
        Err("player not exists".to_string())
    }
}

//落子
#[tauri::command]
pub async fn request_move_later(tab_id: u32, x: u32, y: u32) -> Result<(), String> {
    if let Some(player_socket) = PLAYER_MAP.get(&tab_id) {
        player_socket
            .get_player()
            .await
            .send(Msger::RequestMoveLater.to_msg(format!("{},{}", x, y)))
            .await;
        Ok(())
    } else {
        Err("player not exists".to_string())
    }
}

/// 请求离开房间
#[tauri::command]
pub async fn request_leave_room(tab_id: u32) -> Result<(), String> {
    if let Some(player_socket) = PLAYER_MAP.get(&tab_id) {
        player_socket
            .get_player()
            .await
            .send(Msger::RequestLeaveRoom.to_msg(format!("Ok")))
            .await;
        Ok(())
    } else {
        Err("player not exists".to_string())
    }
}
