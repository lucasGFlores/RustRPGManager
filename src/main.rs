use serde::{Deserialize, Serialize};
use serde_json::{self, Value, Number};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{self, PathBuf}, 
};
mod enums{
    pub mod write_mode;
}
mod utils{
    pub mod save;
}
mod macros;
mod model{ pub mod system_model;
pub mod character_model;
}
use model::{system_model, character_model};
use system_model::SystemModel;
use character_model::Character;
fn main() {

    let system = SystemModel::create("dnd".to_string(), vec![("força".to_string(), Value::Number(Number::from(0))),( "destreza".to_string(), Value::Number(Number::from(0))),("constituição".to_string(), Value::Number(Number::from(0))),("inteligência".to_string(), Value::Number(Number::from(0))),("sabedoria".to_string(), Value::Number(Number::from(0))),("carisma".to_string(), Value::Number(Number::from(0)))
    ]);
    let character = Character::new(system, "dnd".to_string(), "gandalf".to_string());

}
