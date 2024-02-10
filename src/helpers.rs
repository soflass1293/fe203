use crate::{
    db::{get, set},
    todo::{Todo, TodoBuilder, Todos},
};
use std::{cmp::Ordering, time::SystemTime};

pub fn add_todo(title: &str) {
    let mut map = get();
    let todo = TodoBuilder::new(title);
    let id = &todo.id;
    map.insert(id.to_string(), todo);
    set(map);
}

pub fn remove_todo(id: &str) {
    let mut map = get();
    map.remove(id);
    set(map)
}

pub fn toggle_todo(id: &str) {
    let mut map = get();
    if let Some(todo) = map.get_mut(id) {
        todo.completed = !todo.completed;
        todo.updated_at = Some(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        set(map)
    }
}

pub fn get_todos() -> Vec<Todo> {
    let map: std::collections::HashMap<String, Todo> = get();
    let mut vec: Vec<Todo> = map
        .into_iter()
        .map(|(id, todo)| Todo { id, ..todo })
        .collect();
    let _ = &vec.sort_by(|a, b| {
        if a.created_at < b.created_at {
            Ordering::Less
        } else if a.created_at == b.created_at {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    vec
}

pub fn parse(payload: &str) -> Todos {
    serde_json::from_str(payload).unwrap()
}

pub fn stringify(payload: Vec<Todo>) -> String {
    serde_json::to_string(&payload).unwrap()
}
