use std::sync::Mutex;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{listen, tauris::base::APP};

lazy_static!{
    pub static ref TABS: Mutex<Tabs> = Mutex::new(Tabs::default());
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct  Tabs{
    pub tabs: Vec<Tab>,
    counts: u32,
}

impl Tabs {
    //添加主要内容
    pub fn add_main(&mut self){
        self.tabs.push(Tab {
            id: self.counts,
            name: "首页".to_string(),
            mode: "main".to_string(),
        });
        self.counts += 1;
        self.listen();
    }

    pub fn close(&mut self, tab_id: u32){
        self.tabs.retain(|tab| tab.id != tab_id);
        self.listen();
    }

    fn listen(&self){
        listen::change_tabs(APP.get().expect("app initialized filed").clone(), self.clone());
    }
}

impl Default for Tabs {
    fn default() -> Self {
        let mut tabs = Vec::new();
        tabs.push(Tab {
            id: 0,
            name: "首页".to_string(),
            mode: "main".to_string(),
        });
        Self {
            tabs,
            counts: 1,
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Tab{
    pub id: u32,
    pub name: String,
    pub mode: String,
}