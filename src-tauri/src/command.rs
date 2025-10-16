use crate::{
    listen::show_toast,
    player::{self, Data},
    socket::msger::Msger,
    tauris::{base, PLAYER_SOCKET},
};

//必须在启动后就调用
#[tauri::command]
pub fn init_app(app: tauri::AppHandle) {
    if !base::TAURI_INIT.load(std::sync::atomic::Ordering::Relaxed) {
        base::APP.set(app).unwrap();

        base::TAURI_INIT.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

/// 申请展示 toast
#[tauri::command]
pub fn need_show_toast(message: &str, toast_type: &str) {
    show_toast(message, toast_type);
}

/// 刷新数据
#[tauri::command]
pub async fn refresh_data() -> Result<Data, String> {
    if let Some(player_socket) = PLAYER_SOCKET.get(&0) {
        Ok(player_socket.get_player().await.get_data())
    } else {
        Err("player not exists".to_string())
    }
}

#[tauri::command]
pub async fn login(app: tauri::AppHandle, ip: &str, name: &str) -> Result<(), String> {
    let ip = if ip.is_empty() {
        "47.100.88.110:20003"
    } else {
        ip
    };
    let player_socket = player::PlayerSocket::connect(app, ip)
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
    PLAYER_SOCKET.insert(0, player_socket);
    show_toast("登录请求中...", "info");
    Ok(())
}

#[tauri::command]
pub async fn request_enter_room(room_name: &str) -> Result<(), String> {
    if let Some(player_socket) = PLAYER_SOCKET.get(&0) {
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
pub async fn request_be_chess_player(side: &str) -> Result<(), String> {
    if let Some(player_socket) = PLAYER_SOCKET.get(&0) {
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
pub async fn request_leave_seat() -> Result<(), String> {
    if let Some(player_socket) = PLAYER_SOCKET.get(&0) {
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
pub async fn request_admit_defeat() -> Result<(), String> {
    if let Some(player_socket) = PLAYER_SOCKET.get(&0) {
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
pub async fn request_custom_bottom_event(event: &str) -> Result<(), String> {
    if let Some(player_socket) = PLAYER_SOCKET.get(&0) {
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
pub async fn request_move_later(x: u32, y: u32) -> Result<(), String> {
    if let Some(player_socket) = PLAYER_SOCKET.get(&0) {
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
pub async fn request_leave_room() -> Result<(), String> {
    if let Some(player_socket) = PLAYER_SOCKET.get(&0) {
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

/// 开始/结束录制
#[tauri::command]
pub async fn change_reply() -> Result<(), String> {
    if let Some(player_socket) = PLAYER_SOCKET.get(&0) {
        player_socket.get_player().await.change_reply().await;
        Ok(())
    } else {
        Err("player not exists".to_string())
    }
}

/// 名人堂
#[tauri::command]
pub async fn request_player_rank_list() -> Result<(), String> {
    if let Some(player_socket) = PLAYER_SOCKET.get(&0) {
        player_socket
            .get_player()
            .await
            .send(Msger::RequestPlayerRankList.to_msg(format!("Ok")))
            .await;
        Ok(())
    } else {
        Err("player not exists".to_string())
    }
}

/// 热棋榜
#[tauri::command]
pub async fn request_room_rank_list() -> Result<(), String> {
    if let Some(player_socket) = PLAYER_SOCKET.get(&0) {
        player_socket
            .get_player()
            .await
            .send(Msger::RequestRoomRankList.to_msg(format!("Ok")))
            .await;
        Ok(())
    } else {
        Err("player not exists".to_string())
    }
}
