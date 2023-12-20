use crate::enums::write_mode::WriteMode;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
};
#[derive(Debug, Deserialize, Serialize)]
pub struct SystemModel {
    pub name: String,               //name of the system
    pub path: PathBuf,              //path for the .json file of the keys
    pub keys: Vec<(String, Value)>, //values to hold the keys
}
impl SystemModel {
    pub fn load(name: String) -> Result<SystemModel, std::io::ErrorKind> {
        let path: PathBuf = ["maconhaGay", "systems", &format!("{}.json", name)]
            .iter()
            .collect();

        match Self::load_keys(&path) {
            Ok(keys) => Ok(SystemModel { name, path, keys }),
            Err(err) => Err(err),
        }
    }
    pub fn create(name: String, keys: Vec<(String, Value)>) -> SystemModel {
        let path: PathBuf = ["systems", &format!("{}.json", name)].iter().collect();
        SystemModel { name, path, keys }
    }
    pub fn save(&self, option: WriteMode) -> () {
        File::open(&self.path).is_ok();
        let mut file = match option {
            WriteMode::Truncate => OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&self.path)
                .expect("Error opening file"),
            WriteMode::Append => OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&self.path)
                .expect("Error opening file"),
        };
        file.write_all(serde_json::to_string_pretty(self).unwrap().as_bytes())
            .expect("Error writing file");
    }
    pub fn load_keys(path: &PathBuf) -> Result<Vec<(String, Value)>, std::io::ErrorKind> {
        //make a matrix of keys and their types
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => {
                print!("Doesn't exist a file of this system");
                return Err(err.kind());
            }
        };
        let data: Result<SystemModel, serde_json::Error> = serde_json::from_reader(file);
        match data {
            Ok(data) => Ok(data.keys),
            Err(err) => {
                print!("Error keys: {:?}", err.to_string());
                Err(std::io::ErrorKind::WriteZero)
            }
        }
    }
}
