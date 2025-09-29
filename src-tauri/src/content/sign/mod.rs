use dashmap::DashMap;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::content::sign::{
    around_sign::AroundSign,
    badge_sign::BadgeSign,
    cache_sign::{CacheSign, CacheSignKey},
    color_sign::ColorSign,
    ground_sign::GroundSign,
    line_sign::LineSign,
    path_sign::PathSign,
    sign::Sign,
    text_sign::TextSign,
    title_sign::TitleSign,
};

pub mod around_sign;
pub mod badge_sign;
pub mod cache_sign;
pub mod color_sign;
pub mod figure_sign;
pub mod ground_sign;
pub mod line_sign;
pub mod path_sign;
pub mod sign;
pub mod text_sign;
pub mod title_sign;

lazy_static! {
    pub static ref CACHE_MAP: DashMap<CacheSignKey, Vec<SignType>> = DashMap::new();
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignType {
    AroundSign(AroundSign),
    BadgeSign(BadgeSign),
    CacheSign(CacheSign),
    ColorSign(ColorSign),
    GroundSign(GroundSign),
    LineSign(LineSign),
    PathSign(PathSign),
    TextSign(TextSign),
    TitleSign(TitleSign),
}

/// 反序列化 Sign
/// send: 发送 cache_name 去请求服务器cache
pub fn sign_derialize(msg: String, send: impl Fn(String)) -> Vec<SignType> {
    let mut res = Vec::new();
    if msg == "-1" {
        return res;
    }

    let signs: Vec<&str> = msg.split(';').collect();
    for sign in signs {
        let sign_s: Vec<&str> = sign.split(',').collect();
        let sign_type = sign_s[0];
        match sign_type {
            "AroundSign" => {
                if let Ok(around_sign) = AroundSign::deserialize_str(sign_s) {
                    res.push(SignType::AroundSign(around_sign));
                };
            }
            "BadgeSign" => {
                if let Ok(badge_sign) = BadgeSign::deserialize_str(sign_s) {
                    res.push(SignType::BadgeSign(badge_sign));
                };
            }
            "CacheSign" => {
                if let Ok(cache_sign) = CacheSign::deserialize_str(sign_s) {
                    let key = cache_sign.toKey();
                    match CACHE_MAP.get(&key) {
                        Some(value) => {
                            for sign in &*value {
                                res.push(sign.clone());
                            }
                        }
                        None => {
                            send(key.cache_name.clone());
                        }
                    }
                    res.push(SignType::CacheSign(cache_sign));
                };
            }
            "ColorSign" => {
                if let Ok(color_sign) = ColorSign::deserialize_str(sign_s) {
                    res.push(SignType::ColorSign(color_sign));
                };
            }
            "GroundSign" => {
                if let Ok(ground_sign) = GroundSign::deserialize_str(sign_s) {
                    res.push(SignType::GroundSign(ground_sign));
                };
            }
            "LineSign" => {
                if let Ok(line_sign) = LineSign::deserialize_str(sign_s) {
                    res.push(SignType::LineSign(line_sign));
                };
            }
            "PathSign" => {
                if let Ok(path_sign) = PathSign::deserialize_str(sign_s) {
                    res.push(SignType::PathSign(path_sign));
                };
            }
            "TextSign" => {
                if let Ok(text_sign) = TextSign::deserialize_str(sign_s) {
                    res.push(SignType::TextSign(text_sign));
                };
            }
            "TitleSign" => {
                if let Ok(title_sign) = TitleSign::deserialize_str(sign_s) {
                    res.push(SignType::TitleSign(title_sign));
                };
            }
            _ => {}
        }
    }

    res
}
