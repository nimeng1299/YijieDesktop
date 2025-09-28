use std::sync::{atomic::AtomicBool, Mutex, OnceLock};

use crate::player;

pub static TAURI_INIT: AtomicBool = AtomicBool::new(false);

pub static APP: OnceLock<tauri::AppHandle> = OnceLock::new();
