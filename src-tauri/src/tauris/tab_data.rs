use crate::{account::Account, content::hall_room_list::HallRoomList};

#[derive(Debug)]
pub struct TabData {
    mode: String,
    roomdata: HallRoomList,
    account: Account,
}

impl Default for TabData {
    fn default() -> Self {
        Self {
            mode: Modes::default().to_string(),
            roomdata: HallRoomList::default(),
            account: Account::default(),
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
