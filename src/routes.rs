use crate::utils;
use rocket::form::Form;
use std::cmp::Ordering;
use std::fs;
use std::time::SystemTime;
#[path = "./todo.rs"]
mod todo;
use todo::{Todo, Todos};

#[get("/")]
pub fn index() -> String {
    format!("Running")
}

#[get("/api/todos")]
pub fn get_todos() -> String {
    let content =
        fs::read_to_string("src/db.json").expect("Should have been able to read the file");
    let map: Todos = serde_json::from_str(&content).unwrap();
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
    let ser: String = serde_json::to_string(&vec).unwrap();
    format!("{}", ser)
}

#[derive(FromForm, Debug)]
pub struct UserInput<'r> {
    pub title: &'r str,
}

#[post("/api/todos", data = "<user_input>")]
pub fn post_todo(user_input: Form<UserInput<'_>>) -> String {
    let content =
        fs::read_to_string("src/db.json").expect("Should have been able to read the file");
    let mut map: Todos = serde_json::from_str(&content).unwrap();
    let id = utils::id(7);
    map.insert(
        id.clone(),
        Todo {
            id,
            title: user_input.title.to_string(),
            completed: false,
            created_at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            updated_at: None,
        },
    );
    let serialized = serde_json::to_string(&map).unwrap();
    fs::write("src/db.json", serialized.clone()).expect("Unable to write file");
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
    let ser: String = serde_json::to_string(&vec).unwrap();
    format!("{}", ser)
}

#[patch("/api/todos/<id>/completed")]
pub fn toggle_completed(id: String) -> String {
    let content =
        fs::read_to_string("src/db.json").expect("Should have been able to read the file");
    let mut map: Todos = serde_json::from_str(&content).unwrap();
    if let Some(todo) = map.get_mut(&id) {
        todo.completed = !todo.completed;
        todo.updated_at = Some(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        let serialized = serde_json::to_string(&map).unwrap();
        fs::write("src/db.json", serialized.clone()).expect("Unable to write file");
    }
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
    let ser: String = serde_json::to_string(&vec).unwrap();
    format!("{}", ser)
}

#[delete("/api/todos/<id>")]
pub fn delete_todo(id: String) -> String {
    let content =
        fs::read_to_string("src/db.json").expect("Should have been able to read the file");
    let mut map: Todos = serde_json::from_str(&content).unwrap();
    map.remove(&id);
    let serialized = serde_json::to_string(&map).unwrap();
    fs::write("src/db.json", serialized.clone()).expect("Unable to write file");
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
    let ser: String = serde_json::to_string(&vec).unwrap();
    format!("{}", ser)
}
