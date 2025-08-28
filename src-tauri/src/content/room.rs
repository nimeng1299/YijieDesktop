use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Room{
    pub name: String,
    pub black_player: String,
    pub white_player: String,
    pub status: RoomStatus,
    pub spectator: Vec<String>,
    pub player_num: u8,
    pub max_player_num: u8,
    pub is_forbid_chat: bool, //是否禁用聊天
    pub other: String, // 其他信息
}

impl Room{
    pub fn from_msg(msg: String) -> Result<Self> {
        let msgs: Vec<&str> = msg.split('&').collect();
        if msgs.len() == 10{
            Ok(Self{
                name: msgs[0].to_string(),
                black_player: msgs[1].to_string(),
                white_player: msgs[2].to_string(),
                status: RoomStatus::from_i8(msgs[3].parse()?),
                spectator: msgs[5].split(',').map(|s| s.to_string()).collect(),
                player_num: msgs[6].parse()?,
                max_player_num: msgs[7].parse()?,
                is_forbid_chat: msgs[8].parse().unwrap_or(true),
                other: msgs[9].to_string(),
            })
        } else {
            bail!("room msg error")
        }
    }
}

#[derive(Debug, Serialize, Clone, Copy, Deserialize)]
#[repr(i8)]
pub enum RoomStatus{
    Empty = 0,
    NeedBlack = 1,
    NeedWhite = 2,
    Playing = 3,
    Unknow = -1,
}

impl RoomStatus {
    pub fn from_i8(status: i8) -> Self {
        match status {
            0 => Self::Empty,
            1 => Self::NeedBlack,
            2 => Self::NeedWhite,
            3 => Self::Playing,
            _ => Self::Unknow,
        }
    }
}