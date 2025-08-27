use serde::{Deserialize, Serialize};
use anyhow::{bail, Result};

use crate::content::sign::sign::Sign;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSign{
    indexes: Vec<i32>,
    color: String,
}

impl Sign for ColorSign {
    fn deserialize_str(datas: Vec<&str>) -> Result<Self> {
        if datas.len() ==  3{
            let index: Vec<_> = datas[1]
            .split('a')
            .map(|x| x.parse()) // 解析每个元素
            .collect::<Result<_, _>>()?; // 收集结果并处理错误
            Ok(Self{
                indexes: index,
                color: datas[2].to_string(),
            })
        }else{
            bail!("ColorSign 数据格式错误")
        }
    }
}