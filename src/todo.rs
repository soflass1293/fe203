use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: u64,
    pub updated_at: Option<u64>,
}

pub type Todos = HashMap<String, Todo>;