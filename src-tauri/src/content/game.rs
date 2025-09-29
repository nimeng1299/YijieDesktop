use crate::content::{
    board::Board,
    sign::{cache_sign::CacheSign, sign_derialize, SignType, CACHE_MAP},
};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Clone, Deserialize)]
pub struct Game {
    step: i32,
    current_player: i32,
    black_score: i32,
    white_score: i32,
    game_tip: String,
    board: Board,
    sign: Vec<SignType>,
    coord_mode: String,
}

impl Game {
    pub fn from_msg(msg: String, send: impl Fn(String)) -> Result<Self> {
        let datas: Vec<&str> = msg.split('&').collect();
        if datas.len() == 8 {
            let game = Self {
                step: datas[0].parse()?,
                current_player: datas[1].parse()?,
                black_score: datas[2].parse()?,
                white_score: datas[3].parse()?,
                game_tip: datas[4].to_string(),
                board: Board::from_msg(datas[5].to_string()),
                sign: sign_derialize(datas[6].to_string(), send),
                coord_mode: datas[7].to_string(),
            };
            Ok(game)
        } else {
            bail!("数据格式错误, 数据长度: {}", datas.len())
        }
    }

    pub fn add_cache_sign(&mut self, cache_msg: String) {
        let signs = sign_derialize(cache_msg, |_| {});

        for s in &self.sign {
            match s {
                SignType::CacheSign(c) => {
                    CACHE_MAP.insert(c.toKey(), signs.clone());
                    println!("cache: key:{:#?}, value:{:#?}", c.toKey(), signs);
                }
                _ => {}
            }
        }

        self.sign.extend(signs);
    }
}
