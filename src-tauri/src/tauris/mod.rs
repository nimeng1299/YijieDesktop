use std::sync::atomic::AtomicU32;

use anyhow::{bail, Result};
use dashmap::DashMap;
use lazy_static::lazy_static;

use crate::player;

pub mod base;
lazy_static! {
    pub static ref PLAYER_SOCKET: DashMap<u32, player::PlayerSocket> = DashMap::new();
}

/// player socket
pub fn do_player_sockets(f: impl FnOnce(&mut player::PlayerSocket)) {
    if let Some(mut entry) = PLAYER_SOCKET.get_mut(&0) {
        let ps = entry.value_mut();
        f(ps);
    } else {
        eprintln!("can't get mut PLAYER_SOCKET");
    }
}
