use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::content::sign::sign::Sign;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathSign {
    node: Vec<(i32, i32)>, //点集，每个point由“index"+ "w" +"pos”组成， 例如"13w1"，index同其它标记，pos为以这个格子为锚点自身的9个位置,0-8,从左到右从上到下数，点之间用“a”拼接
    line_color: String,    //线颜色
    gr_color: String,      //封闭图形颜色
    size: f64,             //线的宽度比例
    style: i32,            //线条样式 0:实线， 1:虚线
}

impl Sign for PathSign {
    fn deserialize_str(datas: Vec<&str>) -> Result<Self> {
        if datas.len() == 6 {
            let node: Vec<(i32, i32)> = datas[1]
                .split('a')
                .map(|x| {
                    let parts: Vec<&str> = x.split('w').collect();
                    if parts.len() != 2 {
                        bail!("无效的节点格式: {}", x)
                    } else {
                        let first = parts[0]
                            .parse::<i32>()
                            .map_err(|e| anyhow::anyhow!("解析错误: {} - {}", parts[0], e))?;
                        let second = parts[1]
                            .parse::<i32>()
                            .map_err(|e| anyhow::anyhow!("解析错误: {} - {}", parts[1], e))?;
                        Ok((first, second))
                    }
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(Self {
                node,
                line_color: datas[2].to_string(),
                gr_color: datas[3].to_string(),
                size: datas[4].parse()?,
                style: datas[5].parse()?,
            })
        } else {
            bail!("PathSign 数据格式错误")
        }
    }
}
