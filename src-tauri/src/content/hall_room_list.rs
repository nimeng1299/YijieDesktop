use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::content::room::RoomStatus;

#[derive(Debug, Default, Serialize, Clone, Deserialize)]
pub struct HallRoomList {
    pub rooms: Vec<HallRoom>,
}

impl HallRoomList {
    pub fn from_string(msg: String) -> Self {
        let datas: Vec<&str> = msg.split(';').collect();
        let mut rooms = Vec::new();
        for data in datas {
            println!("data: {}", data);
            match HallRoom::from_string(data) {
                Ok(room) => {
                    rooms.push(room);
                }
                Err(e) => {
                    println!("from string error: {}", e);
                }
            }
        }
        // 按照 sort 从大到小排序
        rooms.sort_by_key(|room| -room.sort);
        Self { rooms }
    }

}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct HallRoom {
    pub name: String,
    pub black_player: String,
    pub white_player: String,
    pub status: RoomStatus,
    pub spectator: Vec<String>,
    pub player_num: u8,
    pub max_player_num: u8,
    pub introduction: String, //简介
    pub sort: i32 // 权重
}

impl HallRoom {
    pub fn from_string(msg: &str) -> Result<Self> {
        let datas: Vec<&str> = msg.split('&').collect();
        if datas.len() == 10 {
           let room = Self{
            name: datas[0].to_string(),
            black_player: datas[1].to_string(),
            white_player: datas[2].to_string(),
            status: RoomStatus::from_i8(datas[3].parse()?),
            spectator:  datas[5].split(",").map(|x| x.to_string()).collect(),
            player_num: datas[6].parse()?,
            max_player_num: datas[7].parse()?,
            introduction: datas[8].to_string(),
            sort: datas[9].parse()?,
           };
           Ok(room)
        }else{
            bail!("数据格式错误, 数据长度: {}", datas.len())
        }
    }
}

