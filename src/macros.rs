use std::collections::HashMap;

use serde_json::Value;

#[macro_export]
macro_rules! deserialize_json_to_hashmap {
    ($json:expr) => {{
        match serde_json::from_str::<Value>($json) {
            Ok(value) => value_to_hashmap(&value),
            Err(err) => {
                eprintln!("Erro ao deserializar JSON: {}", err);
                Value::Null // ou trate o erro de outra maneira, se preferir
            }
        }
    }};
}
#[macro_export]
macro_rules! save_file_in_own_directory {
    ($path:expr, $data:expr,$truncate:expr) => {{
        // Cria os diretórios que estão atrás do arquivo
        fs::create_dir_all($path.parent().unwrap()).expect("Error creating directory");
        let mut file = File::open($path)
            .or_else(|_|{
                println!("File doesn't exist"); 
                File::create($path)}) // Tenta abrir se já existe
            .expect("Error opening file");

        // Escreve os dados no arquivo
        file.write(serde_json::to_string(&$data).unwrap().as_bytes())
            .expect("Error writing to file");
    }};
}
fn value_to_hashmap(value: &Value) -> Option<HashMap<String, String>> {
    if let Value::Object(map) = value {
        let mut hashmap = HashMap::new();
        for (key, value) in map.iter() {
            if let Value::String(s) = value {
                hashmap.insert(key.clone(), s.clone());
            } else {
                return None; 
            }
        }
        Some(hashmap)
    } else {
        None 
    }
}
