use std::sync::atomic::AtomicU32;

use anyhow::{bail, Result};
use dashmap::DashMap;
use lazy_static::lazy_static;

use crate::{listen, tauris::tab_data::TabData};

pub mod base;
pub mod tab_data;
pub mod tabs;

static TAB_TITLE_COUNTER: AtomicU32 = AtomicU32::new(0);

lazy_static! {
    static ref TAB_TITLE_MAP: DashMap<u32, String> = DashMap::new();
    static ref TAB_DATA_MAP: DashMap<u32, (TabData, TabState)> = DashMap::new();
}

#[derive(Debug, PartialEq, Eq)]
pub enum TabState {
    Online,
    Close,
}

/// title
/// 创建新的tab，返回 tab id
pub fn create_tab_title() -> u32 {
    let id = TAB_TITLE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    TAB_TITLE_MAP.insert(id, format!("({id}) 新用户"));
    id
}

/// 登录成功，修改用户名，发送事件
pub fn change_tab_title(app: tauri::AppHandle, id: u32, name: String) {
    TAB_TITLE_MAP.insert(id, format!("({id}) {name}"));
    listen::login_success(app, id, name);
}

/// data
pub fn create_tab_datas(id: u32) {
    let data = TabData::default();
    TAB_DATA_MAP.insert(id, (data, TabState::Online));
}

pub fn get_tab_data(id: u32) -> Result<TabData> {
    if let Some(mut entry) = TAB_DATA_MAP.get_mut(&id) {
        let (data, state) = entry.value_mut();
        if *state == TabState::Online {
            Ok(data.clone())
        } else {
            bail!("tab data is deleted")
        }
    } else {
        bail!("can find tab data")
    }
}

pub fn do_tab_datas(id: u32, f: impl FnOnce(&mut TabData)) {
    if let Some(mut entry) = TAB_DATA_MAP.get_mut(&id) {
        let (data, state) = entry.value_mut();
        if *state == TabState::Online {
            f(data);
        }
    } else {
        eprintln!("can't get mut TAB_DATA_MAP");
    }
}
