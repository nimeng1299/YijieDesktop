use serde::{Deserialize, Serialize};

use crate::{
    account::Account,
    content::{game::Game, hall_room_list::HallRoomList, room::Room},
    listen,
    tauris::TabState,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TabData {
    mode: String,
    roomdata: HallRoomList,
    account: Account,
    room: Room,
    game: Game,
    countdown: (i32, i32),
    buttons: Vec<String>,
    can_move: bool,
}

impl TabData {
    pub fn change_mode(&mut self, app: tauri::AppHandle, tab_id: u32, mode: Modes) {
        self.mode = mode.to_string();
        listen::change_mode(app, tab_id, mode.to_string());
    }

    pub fn change_to_hall(&mut self, app: tauri::AppHandle, tab_id: u32, room_list: HallRoomList) {
        self.roomdata = room_list.clone();
        self.change_mode(app.clone(), tab_id, Modes::RoomList);
        listen::change_to_hall(app, tab_id, room_list);
    }

    pub fn change_account(&mut self, app: tauri::AppHandle, tab_id: u32, account: Account) {
        self.account = account.clone();
        listen::change_account(app, tab_id, account);
    }

    pub fn change_to_room(&mut self, app: tauri::AppHandle, tab_id: u32, room: Room) {
        self.room = room.clone();
        self.change_mode(app.clone(), tab_id, Modes::Game);
        listen::change_to_room(app, tab_id, room);
    }

    pub fn update_game(&mut self, app: tauri::AppHandle, tab_id: u32, game: Game) {
        self.game = game.clone();
        listen::update_game(app, tab_id, game);
    }

    pub fn dispatch_custom_bottom(
        &mut self,
        app: tauri::AppHandle,
        tab_id: u32,
        buttons: Vec<String>,
    ) {
        self.buttons = buttons.clone();
        listen::dispatch_custom_bottom(app, tab_id, buttons);
    }

    pub fn refresh_countdown(&mut self, app: tauri::AppHandle, tab_id: u32, countdown: (i32, i32)) {
        self.countdown = countdown;
        listen::refresh_countdown(app, tab_id, countdown);
    }

    pub fn change_move(&mut self, app: tauri::AppHandle, tab_id: u32, can_move: bool) {
        self.can_move = can_move;
        if can_move {
            listen::you_can_move(app, tab_id);
        } else {
            listen::you_not_move(app, tab_id);
        }
    }
}

impl Default for TabData {
    fn default() -> Self {
        Self {
            mode: Modes::default().to_string(),
            roomdata: HallRoomList::default(),
            account: Account::default(),
            room: Room::default(),
            game: Game::default(),
            countdown: (0, 0),
            buttons: vec![],
            can_move: false,
        }
    }
}

//界面 mode
#[derive(Debug)]
pub enum Modes {
    Login,
    RoomList,
    Game,
}

impl Modes {
    fn to_string(&self) -> String {
        let str = match self {
            Self::Login => "login",
            Self::RoomList => "roomlist",
            Self::Game => "game",
        };
        str.to_string()
    }
}

impl Default for Modes {
    fn default() -> Self {
        Self::Login
    }
}
