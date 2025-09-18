use tauri::Emitter;

use crate::{
    account::Account,
    content::{game::Game, hall_room_list::HallRoomList, room::Room},
    tauris::{base::APP, tabs::Tabs},
};

//Toast
#[tauri::command]
pub fn show_toast(message: &str, toast_type: &str) {
    APP.get()
        .unwrap()
        .emit("show_toast", [message, toast_type])
        .unwrap();
}

//标签发生修改
#[tauri::command]
pub fn change_tabs(app: tauri::AppHandle, tabs: Tabs) {
    app.emit("change_tabs", tabs).unwrap();
}

#[tauri::command]
pub fn login_success(app: tauri::AppHandle, tab_id: u32, name: String) {
    app.emit("login_success", (tab_id, name)).unwrap();
}

#[tauri::command]
pub fn change_to_hall(app: tauri::AppHandle, tab_id: u32, room_list: HallRoomList) {
    app.emit("change_to_hall", (tab_id, room_list)).unwrap();
}

#[tauri::command]
pub fn change_to_room(app: tauri::AppHandle, tab_id: u32, room: Room) {
    app.emit("change_to_room", (tab_id, room)).unwrap();
}

#[tauri::command]
pub fn change_account(app: tauri::AppHandle, tab_id: u32, account: Account) {
    app.emit("change_account", (tab_id, account)).unwrap();
}

#[tauri::command]
pub fn update_game(app: tauri::AppHandle, tab_id: u32, game: Game) {
    app.emit("update_game", (tab_id, game)).unwrap();
}

#[tauri::command]
pub fn dispatch_custom_bottom(app: tauri::AppHandle, tab_id: u32, buttons: Vec<String>) {
    app.emit("dispatch_custom_bottom", (tab_id, buttons))
        .unwrap();
}

#[tauri::command]
pub fn refresh_countdown(app: tauri::AppHandle, tab_id: u32, countdown: (i32, i32)) {
    app.emit("refresh_countdown", (tab_id, countdown)).unwrap();
}

#[tauri::command]
pub fn you_can_move(app: tauri::AppHandle, tab_id: u32) {
    println!("you_can_move");
    app.emit("you_can_move", tab_id).unwrap();
}

#[tauri::command]
pub fn you_not_move(app: tauri::AppHandle, tab_id: u32) {
    println!("you_not_move");
    app.emit("you_not_move", tab_id).unwrap();
}
