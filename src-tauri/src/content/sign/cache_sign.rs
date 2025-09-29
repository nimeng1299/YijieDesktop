use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::content::sign::sign::Sign;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSign {
    room_code: String,
    cache_name: String,
    cache_version: i32,
    is_disk_cache: bool,
    sign_content: String,
}

impl CacheSign {
    pub fn toKey(&self) -> CacheSignKey {
        CacheSignKey::new(
            self.room_code.clone(),
            self.cache_name.clone(),
            self.cache_version,
        )
    }
}

impl Sign for CacheSign {
    fn deserialize_str(datas: Vec<&str>) -> Result<Self> {
        if datas.len() == 5 {
            Ok(Self {
                room_code: datas[1].to_string(),
                cache_name: datas[2].to_string(),
                cache_version: datas[3].parse()?,
                is_disk_cache: datas[4].parse()?,
                sign_content: "".to_string(),
            })
        } else {
            bail!("CacheSign 数据格式错误")
        }
    }
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheSignKey {
    pub room_code: String,
    pub cache_name: String,
    pub cache_version: i32,
}

impl CacheSignKey {
    pub fn new(room_code: String, cache_name: String, cache_version: i32) -> Self {
        Self {
            room_code,
            cache_name,
            cache_version,
        }
    }
}
