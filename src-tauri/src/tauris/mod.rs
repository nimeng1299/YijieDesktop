use anyhow::{bail, Result};
use dashmap::DashMap;
use lazy_static::lazy_static;

use crate::tauris::tab_data::TabData;

pub mod base;
pub mod tab_data;
pub mod tabs;

lazy_static! {
    static ref TAB_DATA_MAP: DashMap<u32, (TabData, TabState)> = DashMap::new();
}

#[derive(Debug, PartialEq, Eq)]
pub enum TabState {
    Online,
    Close,
}

pub fn create_tab_datas(id: u32) {
    let data = TabData::default();
    TAB_DATA_MAP.insert(id, (data, TabState::Online));
}

pub fn do_tab_datas(id: u32, f: impl FnOnce(&mut TabData)) -> Result<()> {
    if let Some(mut entry) = TAB_DATA_MAP.get_mut(&id) {
        let (data, state) = entry.value_mut();
        if *state == TabState::Online {
            f(data);
        }
        Ok(())
    } else {
        bail!("can't get mut TAB_DATA_MAP")
    }
}
