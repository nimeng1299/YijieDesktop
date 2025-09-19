use serde::{Deserialize, Serialize};

use crate::{
    account::Account,
    content::{game::Game, hall_room_list::HallRoomList, room::Room},
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
