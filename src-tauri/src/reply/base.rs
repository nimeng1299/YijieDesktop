use std::sync::atomic::AtomicU32;

use dashmap::DashMap;
use lazy_static::lazy_static;

use crate::reply::reply_manager::ReplyManager;

lazy_static! {
    pub static ref REPLY_MAP: DashMap<u32, ReplyManager> = DashMap::new();
    static ref COUNTER: AtomicU32 = AtomicU32::new(0);
}

pub fn init() -> u32 {
    let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    REPLY_MAP.insert(id, ReplyManager::default());
    id
}
