use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub pass: String,
}

pub struct AppState {
    pub info: Mutex<HashMap<String, Info>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            info: Mutex::new(HashMap::new()),
        }
    }

    pub fn get(&self, name: &String) -> Result<Option<Info>, String> {
        // 写法一
        match self.info.lock().unwrap().get(name) {
            Some(info) => Ok(Some(info.to_owned())),
            None => Ok(None),
        }
        // 写法二
        // if let Some(info) = self.info.lock().unwrap().get(name) {
        //     return Ok(Some(info.to_owned()));
        // }
        // Err("err".to_string())
    }

    pub fn add(&self, _info: Info) -> Result<(), String> {
        match self.get(&_info.name) {
            Ok(Some(_)) => Err(format!("name: {} 已存在", _info.name)),
            Ok(None) => {
                self.info
                    .lock()
                    .unwrap()
                    .insert(_info.name.to_string(), _info);
                return Ok(());
            }
            Err(msg) => Err(msg),
        }
    }

    pub fn update(&self, _info: Info) -> Result<(), String> {
        match self.get(&_info.name) {
            Ok(Some(_)) => {
                self.info
                    .lock()
                    .unwrap()
                    .entry(_info.name)
                    .and_modify(|f| f.pass = _info.pass);
                return Ok(());
            }
            Ok(None) => Err(format!("name: {} 不存在", _info.name)),
            Err(msg) => Err(msg),
        }
    }

    pub fn remove(&self, name: String) -> Result<(), String> {
        match self.get(&name) {
            Ok(Some(_)) => {
                self.info.lock().unwrap().remove(&name);
                return Ok(());
            }
            Ok(None) => Err(format!("name: {} 不存在", name)),
            Err(msg) => Err(msg),
        }
    }
}
