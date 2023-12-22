use serde::{de::value, Deserialize, Serialize};
use serde_json::{self, Number, Value};
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{stdin, stdout, Write},
    path::{self, PathBuf}, ops::Add,
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
            ("viadagem".to_string(), Value::Bool(false)),
        ],
    );
    let mut character = Character::new(system, "dnd".to_string(), "gandalf".to_string());
    character.set_all_stats();
    character.save(None);
}
