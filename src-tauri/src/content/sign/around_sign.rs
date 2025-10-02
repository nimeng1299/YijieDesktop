use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::content::sign::sign::Sign;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AroundSign {
    index: Vec<i32>,
    bg_color: String,
    ed_color: String,
    size: f64,
}

impl Sign for AroundSign {
    fn deserialize_str(datas: Vec<&str>) -> Result<Self> {
        if datas.len() == 5 {
            let index: Vec<_> = datas[1]
                .split('a')
                .map(|x| x.parse()) // 解析每个元素
                .collect::<Result<_, _>>()?; // 收集结果并处理错误
            Ok(Self {
                index,
                bg_color: datas[2].to_string(),
                ed_color: datas[3].to_string(),
                size: datas[4].parse()?,
            })
        } else {
            bail!("AroundSign 数据格式错误")
        }
    }
}
