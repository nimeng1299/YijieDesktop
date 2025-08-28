use crate::content::{board::Board, sign::{sign_derialize, SignType}};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Game{
    step: i32,
    current_player: i32,
    black_score: i32,
    white_score: i32,
    game_tip: String,
    board: Board,
    sign: Vec<SignType>,
    coord_mode: String,
}

impl Game{
    pub fn from_msg(msg: String) -> Result<Self>{
        let datas: Vec<&str> = msg.split('&').collect();
        if datas.len() == 8 {
            let game = Self{
                step: datas[0].parse()?,
                current_player: datas[1].parse()?,
                black_score: datas[2].parse()?,
                white_score: datas[3].parse()?,
                game_tip: datas[4].to_string(),
                board: Board::from_msg(datas[5].to_string()),
                sign: sign_derialize(datas[6].to_string()),
                coord_mode: datas[7].to_string(),
            };
            Ok(game)
        }else{
            bail!("数据格式错误, 数据长度: {}", datas.len())
        }
    }
}

