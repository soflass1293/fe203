use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use crate::utils::id;

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: u64,
    pub updated_at: Option<u64>,
}

pub struct TodoBuilder {}

impl TodoBuilder {
    pub fn new(title: &str) -> Todo {
        Todo {
            id: id(7),
            title: title.to_string(),
            completed: false,
            created_at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: None,
        }
    }
}

pub type Todos = HashMap<String, Todo>;
