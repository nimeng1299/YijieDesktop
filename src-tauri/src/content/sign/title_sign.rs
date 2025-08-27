use serde::{Deserialize, Serialize};
use anyhow::{bail, Result};

use crate::content::sign::sign::Sign;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleSign{
    position_x: f32, //棋盘宽度的等比位置
    position_y: f32, //棋盘高度的等比位置
    direction: i32, //文本方向 0:横向居中  1:纵向居中    2:向右延申     3:向下延申
    title: String, //一段文字
    color: String, //文字颜色 "#FFFFFFFF"
    size: f32, //字体大小比例
}

impl Sign for TitleSign {
    fn deserialize_str(datas: Vec<&str>) -> Result<Self> {
        if datas.len() ==  7{
            Ok(Self{
                position_x: datas[1].parse()?,
                position_y: datas[2].parse()?,
                direction: datas[3].parse()?,
                title: datas[4].to_string(),
                color: datas[5].to_string(),
                size: datas[6].parse()?,
            })
        }else{
            bail!("TitleSign 数据格式错误")
        }
    }
}