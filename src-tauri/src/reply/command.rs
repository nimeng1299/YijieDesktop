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

/// 保存
#[tauri::command]
pub async fn reply_save(id: u32) {
    REPLY_MAP.alter(&id, |_, manager| {
        let p = manager.path.clone();
        let mut d = manager.data.clone();
        tokio::spawn(async move {
            let _ = d.save_with_filename(&p).await;
        });
        manager
    });
}

/// 另存为
#[tauri::command]
pub async fn reply_save_as(id: u32, file: &str) -> Result<(), String> {
    let f = PathBuf::from_str(file).map_err(|e| e.to_string())?;
    REPLY_MAP.alter(&id, |_, mut manager| {
        manager.path = f;
        let p = manager.path.clone();
        let mut d = manager.data.clone();
        tokio::spawn(async move {
            d.save_with_filename(&p).await;
        });
        manager
    });
    Ok(())
}

/// 删除
#[tauri::command]
pub async fn reply_delete(id: u32, index: usize) -> Result<Reply, String> {
    let mut res = Err("delete failed".to_string());
    REPLY_MAP.alter(&id, |_, mut manager| {
        res = manager.delete(index).map_err(|e| e.to_string());
        manager
    });
    res
}

/// 撤回
#[tauri::command]
pub async fn reply_undo(id: u32) {
    REPLY_MAP.alter(&id, |_, mut manager| {
        manager.undo();
        manager
    });
}
