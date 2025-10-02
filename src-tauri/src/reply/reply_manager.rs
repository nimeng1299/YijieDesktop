use std::{collections::VecDeque, io::Read, path::PathBuf};

use anyhow::{self, Result};
use std::fs::File;

use crate::content::{game::Game, reply::Reply};

pub struct ReplyManager {
    data: Reply,
    path: PathBuf,
    operation: VecDeque<Game>,
}

impl ReplyManager {
    pub fn open(&mut self, file: PathBuf) -> Result<Reply> {
        let mut f = File::open(&file)?;
        let mut str = String::new();
        f.read_to_string(&mut str)?;
        let datas: Reply = serde_json::from_str(&str)?;
        self.data = datas.clone();
        self.path = file;

        Ok(datas)
    }
}

impl Default for ReplyManager {
    fn default() -> Self {
        Self {
            data: Reply::new(),
            path: PathBuf::new(),
            operation: VecDeque::new(),
        }
    }
}
