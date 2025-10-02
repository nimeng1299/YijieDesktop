use anyhow::{bail, Ok, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Clone, Deserialize)]
pub struct Account {
    pub id_code: String,
    pub nick_name: String,
    pub chat_level: i8,
    pub chat_tip: String,
    pub other_user_info: String,
}

impl Account {
    pub fn from_msg(msg: String) -> Result<Self> {
        let msgs: Vec<&str> = msg.split('&').collect();
        if msgs.len() >= 5 {
            Ok(Self {
                id_code: get_value(msgs[0]),
                nick_name: get_value(msgs[1]),
                chat_level: get_value(msgs[2]).parse().unwrap_or(0),
                chat_tip: get_value(msgs[3]),
                other_user_info: get_value(msgs[4]),
            })
        } else {
            bail!("account msg error")
        }
    }
}

fn get_value(msg: &str) -> String {
    let msgs: Vec<&str> = msg.split(':').collect();
    if msgs.len() >= 2 {
        msgs[1].to_string()
    } else {
        "".to_string()
    }
}
