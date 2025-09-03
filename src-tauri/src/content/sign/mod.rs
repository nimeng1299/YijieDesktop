use serde::{Deserialize, Serialize};

use crate::content::sign::{
    around_sign::AroundSign, badge_sign::BadgeSign, cache_sign::CacheSign, color_sign::ColorSign,
    ground_sign::GroundSign, line_sign::LineSign, path_sign::PathSign, sign::Sign,
    text_sign::TextSign, title_sign::TitleSign,
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

pub fn sign_derialize(msg: String) -> Vec<SignType> {
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
