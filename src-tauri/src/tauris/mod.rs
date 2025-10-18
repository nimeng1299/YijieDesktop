use std::sync::atomic::AtomicU32;

use anyhow::{bail, Result};
use dashmap::DashMap;
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::player;

pub mod base;
pub mod rich_string;

pub use rich_string::rich_to_html;

lazy_static! {
    pub static ref PLAYER_SOCKET: Arc<Mutex<Option<player::PlayerSocket>>> =
        Arc::new(Mutex::new(None));
}

pub fn do_player_socket(f: impl FnOnce(&mut player::PlayerSocket)) {
    if let Ok(mut global) = PLAYER_SOCKET.try_lock() {
        if let Some(ref mut socket) = *global {
            f(socket);
        }
    }
}
