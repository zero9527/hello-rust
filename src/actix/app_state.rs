use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
    pub name: String,
    pub pass: String,
}

pub struct AppState {
    pub info: HashMap<String, Info>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            info: HashMap::new(),
        }
    }

    pub fn get(&self, name: &String) -> Option<&Info> {
        self.info.get(name)
    }

    pub fn add(&mut self, _info: Info) -> Result<String, String> {
        if let None = self.get(&_info.name) {
            let Info { name, pass } = _info;
            self.info.insert(name.clone(), Info { name, pass });
            return Ok("ok".to_string());
        }
        Err("err".to_string())
    }

    pub fn update(&mut self, _info: Info) -> Result<String, String> {
        if let Some(_) = self.get(&_info.name) {
            self.info
                .entry(_info.name)
                .and_modify(|f| f.pass = _info.pass);
            return Ok("ok".to_string());
        }
        Err("error".to_string())
    }

    pub fn remove(&mut self, name: String) -> Result<String, String> {
        if let Some(_) = self.get(&name) {
            self.info.remove(&name);
            return Ok("ok".to_string());
        }
        Err("err".to_string())
    }
}
