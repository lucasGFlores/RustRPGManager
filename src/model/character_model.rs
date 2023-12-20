use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::save_file_in_own_directory;

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
    
  pub fn save(&self) -> () {
        let path: PathBuf = [&self.rpg_name, &format!("{}.json", &self.name)].iter().collect();
       save_file_in_own_directory!(&path, self,true);
    }
   fn set_status(&mut self) -> () {
      self.system.keys.iter().map(|(key, value)| {
            let mut status: HashMap<String, Value> = HashMap::new();
            status.insert(key.to_string(), value.clone());
            status
        }); // Convert the map iterator to a vector
    }
    fn get_status_pattern(&self) -> Vec<(String,String)> {
        self.system.keys.iter().map(|(key, value)| {
            (key.to_string(), value.name_of_value())
        }).collect()
    }
}

trait NameOfValue {
    fn name_of_value(&self) -> String;
}

impl NameOfValue for Value {
    fn name_of_value(&self) -> String {
        match self {
            Value::String(string) => "texto".to_string(),
            Value::Number(Number) => "numero".to_string(),
            Value::Bool(bool) => "sim/não".to_string(),
            _ => "não sei".to_string(),
        }
    }
}