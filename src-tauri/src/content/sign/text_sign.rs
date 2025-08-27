use serde::{Deserialize, Serialize};
use anyhow::{bail, Result};

use crate::content::sign::sign::Sign;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSign{
    index: i32, //格子位置，从左往右-从上往下一个个数的序号。
    value: String, //单个文字
    color: String, //颜色 "#FFFFFFFF"
    angle: i32, //旋转角度 0-360°
}

impl Sign for TextSign {
    fn deserialize_str(datas: Vec<&str>) -> Result<Self> {
        if datas.len() ==  5{
            Ok(Self{
                index: datas[1].parse()?,
                value: datas[2].to_string(),
                color: datas[3].to_string(),
                angle: datas[4].parse()?,
            })
        }else{
            bail!("TextSign 数据格式错误")
        }
    }
}