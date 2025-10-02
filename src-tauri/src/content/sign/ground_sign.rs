use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::content::sign::sign::Sign;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundSign {
    start: i32,
    end: i32,
    bg_color: String,
    ed_color: String,
    size: f64,
    bg_style: i32,
    ed_style: i32,
}

impl Sign for GroundSign {
    fn deserialize_str(datas: Vec<&str>) -> Result<Self> {
        if datas.len() == 8 {
            Ok(Self {
                start: datas[1].parse()?,
                end: datas[2].parse()?,
                bg_color: datas[3].to_string(),
                ed_color: datas[4].to_string(),
                size: datas[5].parse()?,
                bg_style: datas[6].parse()?,
                ed_style: datas[7].parse()?,
            })
        } else {
            bail!("GroundSign 数据格式错误")
        }
    }
}
