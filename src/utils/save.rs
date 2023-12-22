use std::{
    fs::{File, OpenOptions, create_dir},
    io::Write,
    path::PathBuf,
};

use crate::enums::write_mode::WriteMode;

pub fn save_file_in_own_directory(json_value: String, path: &PathBuf, option: Option<WriteMode>) -> () {
      let option = if let Some(option) = option {
            option
        } else {
            WriteMode::Append
        };
    if File::open(&path).is_ok(){
    let mut file = match option {
        WriteMode::Truncate => OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .expect("Error opening file"),
        WriteMode::Append => OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&path)
            .expect("Error opening file"),
    };
    file.write_all(json_value.as_bytes())
        .expect("Error writing file");
} else {
    create_dir(path.parent().unwrap()).unwrap();
    File::create(path).unwrap();
    save_file_in_own_directory(json_value, path, Some(option));

}
}