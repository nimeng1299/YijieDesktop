use tauri::Emitter;

use crate::{account::Account, content::{hall_room_list::HallRoomList, room::Room}, tauris::tabs::Tabs};

//标签发生修改
#[tauri::command]
pub fn change_tabs(app: tauri::AppHandle,  tabs: Tabs){
    app.emit("change_tabs", tabs).unwrap();
}

#[tauri::command]
pub fn login_success(app: tauri::AppHandle,  tab_id: u32, name: String){
    app.emit("login_success", (tab_id, name)).unwrap();
}

#[tauri::command]
pub fn change_to_hall(app: tauri::AppHandle, tab_id: u32, room_list: HallRoomList){
    app.emit("change_to_hall", (tab_id, room_list)).unwrap();
}

#[tauri::command]
pub fn change_to_room(app: tauri::AppHandle, tab_id: u32, room: Room){
    app.emit("change_to_room", (tab_id, room)).unwrap();
}

#[tauri::command]
pub fn change_account(app: tauri::AppHandle, tab_id: u32, account: Account){
    app.emit("change_account", (tab_id, account)).unwrap();
}