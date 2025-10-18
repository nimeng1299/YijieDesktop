pub mod account;
pub mod api;
pub mod command;
pub mod content;
pub mod listen;
pub mod player;
pub mod reply;
pub mod socket;
pub mod tauris;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            command::init_app,
            command::need_show_toast,
            command::login,
            command::refresh_data,
            command::request_enter_room,
            command::request_be_chess_player,
            command::request_leave_seat,
            command::request_admit_defeat,
            command::request_custom_bottom_event,
            command::request_move_later,
            command::request_leave_room,
            command::change_reply,
            command::request_player_rank_list,
            command::request_room_rank_list,
            command::request_chess_statistics,
            command::request_chess_rule,
            listen::show_toast,
            listen::login_success,
            listen::change_mode,
            listen::change_to_hall,
            listen::change_to_room,
            listen::change_account,
            listen::update_game,
            listen::dispatch_custom_bottom,
            listen::refresh_countdown,
            listen::you_can_move,
            listen::you_not_move,
            listen::is_start_reply,
            listen::show_info_dialog,
            listen::update_deloy,
            reply::command::reply_init,
            reply::command::reply_open,
            reply::command::reply_save,
            reply::command::reply_save_as,
            reply::command::reply_delete,
            reply::command::reply_undo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
