use serde::{de::value, Deserialize, Serialize};
use serde_json::{self, Number, Value};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{stdin, stdout, Write},
    path::{self, PathBuf},
};
mod enums {
    pub mod write_mode;
}
mod utils {
    pub mod save;
}
mod macros;
mod model {
    pub mod character_model;
    pub mod system_model;
}
use character_model::Character;
use model::{character_model, system_model};
use system_model::SystemModel;
fn main() {
    let system = SystemModel::create(
        "dnd".to_string(),
        vec![
            ("força".to_string(), Value::Number(Number::from(0))),
            ("destreza".to_string(), Value::Number(Number::from(0))),
            ("constituição".to_string(), Value::Number(Number::from(0))),
            ("inteligência".to_string(), Value::Number(Number::from(0))),
            ("sabedoria".to_string(), Value::Number(Number::from(0))),
            ("carisma".to_string(), Value::Number(Number::from(0))),
        ],
    );
    let character = Character::new(system, "dnd".to_string(), "gandalf".to_string());
    // print!("{:?}",character.get_status_pattern());
    // setar os status do user
    let mut values: String = String::new();
    let mut stats: HashMap<String, Value> = HashMap::new();
    for stats in character.get_stats_pattern().iter() {
        print!("\ncoloque o valor para {} ({})\n", stats.0, stats.1);
        match stdin().read_line(&mut values) {
            Ok(n) => {
                print!("\n");
                
            } //make mitigation value type error
            Err(_) => {
                print!("have problem in write the value of {}", stats.0);
                break;
            }
        }
    }

    //    stdin()
}
