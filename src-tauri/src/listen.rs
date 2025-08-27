use tauri::Emitter;

use crate::{content::hall_room_list::HallRoomList, tauris::tabs::Tabs};

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