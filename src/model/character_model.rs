use std::{collections::HashMap, path::PathBuf};

use clap::builder::Str;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{enums::write_mode::WriteMode, utils::save::save_file_in_own_directory};

use super::system_model::SystemModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct Character {
    system: SystemModel,
    rpg_name: String,
    name: String,
    status: Option<HashMap<String, Value>>,
}
impl Character {
    pub fn new(system: SystemModel, rpg_name: String, name: String) -> Character {
        Character {
            system,
            rpg_name,
            name,
            status: None,
        }
    }

    pub fn save(&self, option: Option<WriteMode>) -> () {
        let path: PathBuf = [&self.rpg_name, &format!("{}.json", &self.name)]
            .iter()
            .collect();
        save_file_in_own_directory(serde_json::to_string_pretty(self).unwrap(), &path, option);
    }
    fn set_stats(&mut self,values : String) -> () {
        
        self.system.keys.iter().map(|(key, value)| {
            let mut status: HashMap<String, Value> = HashMap::new();
            status.insert(key.to_string(), value.clone());
            status
        }); // Convert the map iterator to a vector
    }
    pub fn get_stats_pattern(&self) -> Vec<(String, String)> {
        self.system
            .keys
            .iter()
            .map(|(key, value)| (key.to_string(), value.value_of()))
            .collect()
    }
}

trait ValueOf {
    fn value_of(&self) -> String;
}

impl ValueOf for Value {
    fn value_of(&self) -> String {
        match self {
            Value::String(string) => "texto".to_string(),
            Value::Number(Number) => "numero".to_string(),
            Value::Bool(bool) => "sim/não".to_string(),
            _ => "não sei".to_string(),
        }
    }
}
