use std::{path::PathBuf, str::FromStr};

use crate::{
    content::reply::Reply,
    reply::base::{init, REPLY_MAP},
};

/// 新建窗口，返回 id
#[tauri::command]
pub fn reply_init() -> u32 {
    init()
}

/// 打开一个文件，返回加载的 Reply
#[tauri::command]
pub fn reply_open(id: u32, file: &str) -> Result<Reply, String> {
    let f = PathBuf::from_str(file).map_err(|e| e.to_string())?;
    let mut reply = Err("not find reply".to_string());
    REPLY_MAP.alter(&id, |_, mut manager| {
        reply = manager.open(f).map_err(|e| e.to_string());

        manager
    });
    reply
}
