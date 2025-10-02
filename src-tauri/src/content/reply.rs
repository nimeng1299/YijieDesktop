use anyhow::{self, Ok, Result};
use serde::{Deserialize, Serialize};

use crate::{
    api::{get_current_exe_dir, get_foramt_time, save_file},
    content::game::Game,
};

/// 录制棋盘回放
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reply {
    title: String,
    black_player: String,
    white_player: String,
    time: String,
    #[serde(default = "version")]
    version: String,
    datas: Vec<Game>,
}

impl Reply {
    pub fn new() -> Self {
        Self {
            title: "".to_string(),
            black_player: "".to_string(),
            white_player: "".to_string(),
            time: get_foramt_time(),
            version: version(),
            datas: vec![],
        }
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_black_player(&mut self, name: String) {
        self.black_player = name;
    }

    pub fn set_white_player(&mut self, name: String) {
        self.white_player = name;
    }

    /// 添加一步
    pub fn add_board(&mut self, game: Game) {
        self.datas.push(game);
    }

    /// 保存到文件，同时情况里面存储的内容
    pub async fn save(&mut self) -> Result<String> {
        let file = get_current_exe_dir()?
            .join("reply")
            .join(format!("{}-{}.json", self.title, self.time));
        let str;
        if let Some(s) = file.to_str() {
            str = s.to_string();
        } else {
            str = format!("{}-{}.json", self.title, self.time);
        }

        save_file(file, serde_json::to_string(self)?).await?;
        self.datas = vec![];

        Ok(str)
    }
}

impl Drop for Reply {
    fn drop(&mut self) {
        if self.datas.len() > 0 {
            let _ = self.save();
        }
    }
}

/// 当前 Reply 版本
fn version() -> String {
    "1.0.0".to_string()
}
