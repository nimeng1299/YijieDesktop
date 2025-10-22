use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::content::sign::sign::Sign;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigureSign {
    tiles: Vec<(i32, i32)>, // 瓦片集合，每个元素由“start"+"t"+"end”组合，start格子的左上角和end格子的右下角表示一个瓦片的矩形范围。集合元素之间用"a"拼接
    color: String,          // 颜色 "#FFFFFFFF"

    mid_side_line: String, // 四边中点到四角的边线：长度为 8，左上左半边-左上右半边-右上左半边------左下右半边-左下左半边,0或1来表示开关
    mid_cen_line: String,  // 四边中点到正中心的内线：长度为 4，上右下左，0 或 1 来表示开关
    cor_cen_line: String,  // 四角到正中心的内线：长度为 4，左上为起点顺时针，0 或 1 来表示开关
    adj_mid_line: String, // 相邻两个四边中点的内连线：长度为 4，左上为起点顺时针，0 或 1 来表示开关
    cor_mid_line: String, // 四角到对面边的中点的内连线：长度为 8，左上角为起点顺时针，0 或 1 来表示开关

    comp_cen_arc: String, // 整个瓦片矩形内切椭圆的四分之一弧：长度为4，左上为起点顺时针，0 或 1 来表示开关
    cen_exp_arc: String, // 以四边为对称轴的扩大矩形的且与中心点相切的四分之一弧线：长度为 8，左上左半边-左上右半边-右上左半边------左下右半边-左下左半边,0 或 1 来表示开关
    cor_exp_arc: String, // 以四个角为正中心的扩大矩形的内切椭圆的四分之一弧：长度为 4，左上为起点顺时针，0 或 1 来表示开关
    cor_cen_arc: String, // 以四个角为正中心的中心对称矩形的四分之一弧线：长度为 4，左上为起点顺时针，0 或 1 来表示开关
    side_exp_arc: String, // 以四边为对称轴的扩大矩形的内切椭圆的四分之一弧线：长度为 8，左上左半边-左上右半边-右上左半边------左下右半边-左下左半边,0 或 1 来表示开关
    cor_mid_arc: String, // 一个扩大矩形，以某个角为一边中心，那个角对面的边的中点为 扩大矩形另一相邻边的的中心 的内切椭圆的四分之一弧：长度为8，左上左半边-左上右半边-右上左半边------左下右半边-左下左半边,0 或 1 来表示开关

    comp_point: String, // 整个瓦片上边角中心位置的点：长度为 1，左上角开始顺时针-最后到中心点，0 和 1 来表示开关
    comp_angle: String, // 整个瓦片上四个角偏内部一点和中心项四个角偏外一点的直角：长度为8，先是四个角向内左上为起点顺时针4个，然后是内部向四个角左上为起点顺时针4个，0 或 1 来表示开关
}

impl Sign for FigureSign {
    fn deserialize_str(datas: Vec<&str>) -> Result<Self> {
        if datas.len() == 16 {
            let tiles = datas[1]
                .split('a')
                .filter(|s| !s.is_empty()) // 去掉可能的空字符串
                .filter_map(|part| {
                    let mut nums = part.split('t');
                    let first = nums.next()?.parse::<i32>().ok()?;
                    let second = nums.next()?.parse::<i32>().ok()?;
                    Some((first, second))
                })
                .collect();

            Ok(Self {
                tiles,
                color: datas[2].to_string(),
                mid_side_line: datas[3].to_string(),
                mid_cen_line: datas[4].to_string(),
                cor_cen_line: datas[5].to_string(),
                adj_mid_line: datas[6].to_string(),
                cor_mid_line: datas[7].to_string(),
                comp_cen_arc: datas[8].to_string(),
                cen_exp_arc: datas[9].to_string(),
                cor_exp_arc: datas[10].to_string(),
                cor_cen_arc: datas[11].to_string(),
                side_exp_arc: datas[12].to_string(),
                cor_mid_arc: datas[13].to_string(),
                comp_point: datas[14].to_string(),
                comp_angle: datas[15].to_string(),
            })
        } else {
            bail!("FigureSign 数据格式错误")
        }
    }
}
