use std::sync::{atomic::AtomicBool, OnceLock};

pub static TAURI_INIT:AtomicBool = AtomicBool::new(false);

pub static APP: OnceLock<tauri::AppHandle> = OnceLock::new();
