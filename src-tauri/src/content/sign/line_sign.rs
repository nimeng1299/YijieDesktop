use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::content::sign::sign::Sign;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineSign {
    start: i32,
    end: i32,
    color: String,
    style: i32, //0:普通线条  1:单向箭头  2:双向箭头
}

impl Sign for LineSign {
    fn deserialize_str(datas: Vec<&str>) -> Result<Self> {
        if datas.len() == 5 {
            Ok(Self {
                start: datas[1].parse()?,
                end: datas[2].parse()?,
                color: datas[3].to_string(),
                style: datas[4].parse()?,
            })
        } else {
            bail!("LineSign 数据格式错误")
        }
    }
}
