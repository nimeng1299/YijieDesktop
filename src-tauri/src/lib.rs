pub mod player;
pub mod account;
pub mod socket;
pub mod command;
pub mod listen;
pub mod content;
pub mod tauris;



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            command::init_app,
            command::add_tab_main,
            command::close_tab,
            command::login, 
            command::close, 
            listen::login_success,
            listen::change_tabs,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
