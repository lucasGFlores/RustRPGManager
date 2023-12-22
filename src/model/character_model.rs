use std::{collections::HashMap, io::stdin, path::PathBuf};

use clap::builder::Str;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use crate::{enums::write_mode::WriteMode, utils::save::save_file_in_own_directory};

use super::system_model::SystemModel;
use crate::model::character_model::dataTypes::{DBool, DNumber, DString};

#[derive(Debug, Deserialize, Serialize)]
pub struct Character {
    system: SystemModel,
    rpg_name: String,
    name: String,
    status: Option<Vec<Value>>,
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
    pub fn set_all_stats(&mut self) -> () {
        let mut values: String = String::new();
        let mut stats: Vec<Value> = Vec::new();
        for stat in self.get_stats_pattern().iter() {
            print!(
                "\ncoloque o valor para {} ({})\n",
                stat.0,
                stat.1.value_of()
            );
            loop {
                match stdin().read_line(&mut values) {
                    Ok(_n) => {
                        // print!("\n");
                        values = format!("{}", values.trim());
                        // print!("\n\n{}\n",values.trim());
                        let last_value = values.clone();
                        values = String::new();
                        // print!("\nsexo \n");
                        match &stat.1 {
                            Value::Number(n) => match last_value.parse::<u32>() {
                                Ok(value) => {
                                    let value = Value::Number(Number::from(value));
                                    stats.push(value);
                                    break;
                                }
                                Err(_) => {
                                    print!("valor invalido, coloque um numero\n")
                                }
                            },
                            Value::Bool(_b) => {
                                //atualizar escolhas linguagem...
                                match last_value.to_lowercase().as_str() {
                                    "sim" | "s" | "yes" | "y" => {
                                        let value = Value::Bool(true);
                                        stats.push(value);
                                        break;
                                    }
                                    "não" | "n" | "no" => {
                                        let value = Value::Bool(false);
                                        stats.push(value);
                                        break;
                                    }
                                    _ => {
                                        print!("valor invalido, coloque sim ou não\n");
                                    }
                                }
                            }
                            Value::Null => stats.push(Value::Null),
                            Value::String(_) => todo!(),
                            Value::Array(_) => todo!(),
                            Value::Object(_) => todo!(),
                        }
                    }
                    Err(_) => {
                        print!("have problem in write the value of {}", stat.0);
                        break;
                    }
                }
            }
        }
    }
    fn get_stats_pattern(&self) -> Vec<(String, Value)> {
        self.system
            .keys
            .iter()
            .map(|(key, value)| (key.to_string(), value.clone()))
            .collect()
    }
    fn get_stats(&self, value: dataTypes) -> dataTypes {
        match value {
            DString(string) => {
                let value = self.system.keys.get(&string).unwrap().clone();
            }
            DNumber(number) => {
                let value = Value::Number(Number::from(number));
                dataTypes::DNumber(number)
            }
        }
    }
}

enum dataTypes {
    DString(String),
    DNumber(u32),
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
