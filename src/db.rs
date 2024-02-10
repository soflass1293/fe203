use std::fs;

use crate::{todo::Todos, helpers::parse};

const DB_PATH: &str = "src/db.json";

pub fn get() -> Todos {
    if fs::metadata(DB_PATH).is_err() {
        fs::write(DB_PATH, "{}").expect("Unable to write file");
    }
    let payload = fs::read_to_string(DB_PATH).expect("Should have been able to read the file");
    parse(&payload)
}

pub fn set(todos: Todos) {
    let serialized = serde_json::to_string(&todos).unwrap();
    fs::write(DB_PATH, serialized).expect("Unable to write file")
}
