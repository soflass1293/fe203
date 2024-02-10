#[macro_use]
extern crate rocket;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::form::Form;
use rocket::http::Header;
use rocket::{Request, Response};
use serde::{Deserialize, Serialize};
use rand::Rng;
use std::collections::HashMap;
use std::fs;
use std::iter;
use std::time::SystemTime;

fn id(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    id: String,
    title: String,
    completed: bool,
    created_at: u64,
    updated_at: Option<u64>,
}

type Todos = HashMap<String, Todo>;

#[get("/")]
fn index() -> String {
    format!("Running")
}

#[get("/api/todos")]
fn get_todos() -> String {
    let content =
        fs::read_to_string("src/db.json").expect("Should have been able to read the file");
    let map: Todos = serde_json::from_str(&content).unwrap();
    let vec: Vec<Todo> = map
        .into_iter()
        .map(|(id, todo)| Todo { id, ..todo })
        .collect();
    let ser = serde_json::to_string(&vec).unwrap();
    format!("{}", ser)
}

#[derive(FromForm, Debug)]
struct UserInput<'r> {
    title: &'r str,
}

#[post("/api/todos", data = "<user_input>")]
fn post_todo(user_input: Form<UserInput<'_>>) -> String {
    println!("{:#?}", user_input);
    let content =
        fs::read_to_string("src/db.json").expect("Should have been able to read the file");
    let mut map: Todos = serde_json::from_str(&content).unwrap();
    let id = id(7);
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
    let vec: Vec<Todo> = map
        .into_iter()
        .map(|(id, todo)| Todo { id, ..todo })
        .collect();
    let ser = serde_json::to_string(&vec).unwrap();
    format!("{}", ser)
}

#[patch("/api/todos/<id>/completed")]
fn toggle_completed(id: String) -> String {
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
    let vec: Vec<Todo> = map
        .into_iter()
        .map(|(id, todo)| Todo { id, ..todo })
        .collect();
    let ser = serde_json::to_string(&vec).unwrap();
    format!("{}", ser)
}

#[delete("/api/todos/<id>")]
fn delete_todo(id: String) -> String {
    let content =
        fs::read_to_string("src/db.json").expect("Should have been able to read the file");
    let mut map: Todos = serde_json::from_str(&content).unwrap();
    map.remove(&id);
    let serialized = serde_json::to_string(&map).unwrap();
    fs::write("src/db.json", serialized.clone()).expect("Unable to write file");
    let vec: Vec<Todo> = map
        .into_iter()
        .map(|(id, todo)| Todo { id, ..todo })
        .collect();
    let ser = serde_json::to_string(&vec).unwrap();
    format!("{}", ser)
}

#[launch]
fn rocket() -> _ {
    if fs::metadata("src/db.json").is_err() {
        fs::write("src/db.json", "{}").expect("Unable to write file");
    }
    rocket::build().attach(Cors).mount(
        "/",
        routes![
            index,
            all_options,
            get_todos,
            post_todo,
            toggle_completed,
            delete_todo
        ],
    )
}

#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

pub struct Cors;
#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
