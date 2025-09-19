use dashmap::DashMap;
use lazy_static::lazy_static;

use crate::tauris::tab_data::TabData;

pub mod base;
pub mod tab_data;
pub mod tabs;

lazy_static! {
    pub static ref TAB_DATA_MAP: DashMap<u32, (TabData, TabState)> = DashMap::new();
}

pub enum TabState {
    Online,
    Close,
}
