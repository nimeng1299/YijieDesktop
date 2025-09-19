use serde::{Deserialize, Serialize};

use crate::{
    account::Account,
    content::{game::Game, hall_room_list::HallRoomList, room::Room},
    listen,
};

#[derive(Debug, Serialize, Deserialize)]
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
    pub fn change_to_hall(&mut self, app: tauri::AppHandle, tab_id: u32, room_list: HallRoomList) {
        self.roomdata = room_list.clone();
        listen::change_to_hall(app, tab_id, room_list);
    }
    pub fn change_account(&mut self, app: tauri::AppHandle, tab_id: u32, account: Account) {
        self.account = account.clone();
        listen::change_account(app, tab_id, account);
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
