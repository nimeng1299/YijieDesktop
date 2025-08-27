use serde::{Deserialize, Serialize};
use anyhow::{bail, Result};

use crate::content::sign::sign::Sign;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BadgeSign{
    index: i32,
    value: String,
    position: i32,
    te_color: String,
    bg_color: String,
}

impl Sign for BadgeSign {
    fn deserialize_str(datas: Vec<&str>) -> Result<Self> {
        if datas.len() ==  6{
            Ok(Self{
                index: datas[1].parse()?,
                value: datas[2].to_string(),
                position: datas[3].parse()?,
                te_color: datas[4].to_string(),
                bg_color: datas[5].to_string(),
            })
        }else{
            bail!("BadgeSign 数据格式错误")
        }
    }
}