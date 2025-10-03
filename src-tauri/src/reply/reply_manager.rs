use std::{collections::VecDeque, io::Read, path::PathBuf};

use anyhow::{self, bail, Ok, Result};
use std::fs::File;

use crate::content::{game::Game, reply::Reply};

pub struct ReplyManager {
    pub data: Reply,
    pub path: PathBuf,
    operation: VecDeque<Reply>,
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

    pub fn delete(&mut self, index: usize) -> Result<Reply> {
        self.operation.push_back(self.data.clone());
        if self.data.remove_item(index) {
            Ok(self.data.clone())
        } else {
            self.operation.pop_front();
            bail!("can delete data from index: {index}")
        }
    }

    pub fn undo(&mut self) {
        if let Some(reply) = self.operation.pop_front() {
            self.data = reply;
        }
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
