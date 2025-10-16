use tauri::Emitter;

use crate::{
    account::Account,
    content::{game::Game, hall_room_list::HallRoomList, room::Room},
    tauris::base::APP,
};

//Toast
#[tauri::command]
pub fn show_toast(message: &str, toast_type: &str) {
    APP.get()
        .unwrap()
        .emit("show_toast", [message, toast_type])
        .unwrap();
}

#[tauri::command]
pub fn login_success(app: tauri::AppHandle, name: String) {
    app.emit("login_success", name).unwrap();
}

/// 用户位置改变 登录/大厅/房间/...
#[tauri::command]
pub fn change_mode(app: tauri::AppHandle, mode: String) {
    app.emit("change_mode", mode).unwrap();
}

#[tauri::command]
pub fn change_to_hall(app: tauri::AppHandle, room_list: HallRoomList) {
    app.emit("change_to_hall", room_list).unwrap();
}

#[tauri::command]
pub fn change_to_room(app: tauri::AppHandle, room: Room) {
    app.emit("change_to_room", room).unwrap();
}

#[tauri::command]
pub fn change_account(app: tauri::AppHandle, account: Account) {
    app.emit("change_account", account).unwrap();
}

/// 棋盘更新
#[tauri::command]
pub fn update_game(app: tauri::AppHandle, game: Game) {
    app.emit("update_game", game).unwrap();
}

/// 按钮渲染
#[tauri::command]
pub fn dispatch_custom_bottom(app: tauri::AppHandle, buttons: Vec<String>) {
    app.emit("dispatch_custom_bottom", buttons).unwrap();
}

/// 倒计时更新
#[tauri::command]
pub fn refresh_countdown(app: tauri::AppHandle, countdown: (i32, i32)) {
    app.emit("refresh_countdown", countdown).unwrap();
}

#[tauri::command]
pub fn you_can_move(app: tauri::AppHandle) {
    println!("you_can_move");
    app.emit("you_can_move", ()).unwrap();
}

#[tauri::command]
pub fn you_not_move(app: tauri::AppHandle) {
    println!("you_not_move");
    app.emit("you_not_move", ()).unwrap();
}

#[tauri::command]
pub fn is_start_reply(app: tauri::AppHandle, is_start: bool) {
    println!("is_start_reply, {is_start}");
    app.emit("is_start_reply", is_start).unwrap();
}

#[tauri::command]
pub fn show_info_dialog(app: tauri::AppHandle, data: String) {
    app.emit("show_info_dialog", data).unwrap();
}

/// 更新延迟(ms)
#[tauri::command]
pub fn update_deloy(app: tauri::AppHandle, deloy: i64) {
    app.emit("update_deloy", deloy).unwrap();
}
