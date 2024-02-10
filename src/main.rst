#[macro_use]
extern crate rocket;
use rand::Rng;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use std::fs;
use std::iter;

fn id(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    title: String,
    completed: bool,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct Todos {
    #[serde_as(as = "Vec<(_, _)>")]
    todos: HashMap<String, Todo>,
}
trait ITodos {
    fn add(todo: Todo) -> HashMap<String, Todo>;
}
impl Todos {
    fn new() -> Todos {
        Todos {
            todos: HashMap::new(),
        }
    }
    fn add(&mut self, title: String) {
        let todo = Todo {
            title,
            completed: false,
        };
        let id = id(10);
        self.todos.insert(id, todo);
    }
}

#[get("/")]
fn index() -> String {
    let content =
        fs::read_to_string("src/todos.json").expect("Should have been able to read the file");
    format!("{}", content)
}
#[get("/api/todos")]
fn get_todos() -> String {
    let content =
        fs::read_to_string("src/todos.json").expect("Should have been able to read the file");
    format!("{}", content)
}
#[post("/api/todos/<title>")]
fn post_todo(title: String) -> String {
    let content =
        fs::read_to_string("src/todos.json").expect("Should have been able to read the file");
    format!("{}", content)
}
#[patch("/api/todos/<id>")]
fn toggle_completed(id: String) -> String {
    // let content =
    //     fs::read_to_string("src/todos.json").expect("Should have been able to read the file");
    // let deserialized: Todo = serde_json::from_str(&content).unwrap();
    // print!(deserialized);
    format!("xxx")
}

#[launch]
fn rocket() -> _ {
    // let content =
    //     fs::read_to_string("src/todos.json").expect("Should have been able to read the file");
    // let j = "
    //     {
    //       \"todos\": {
    //         \"F9BA143B95FF6D82\": {
    //         \"id\": \"F9BA143B95FF6D82\",
    //         \"title\": \"Menlo Park, CA\",
    //         \"completed\": false
    //       }
    //     }
    //     }";
    // let deserialized: Todos = serde_json::from_str(j).unwrap();
    // print!("BOOOOOO");
    // print!("{:?}", deserialized);
    // print!("BOOOOOO");
    rocket::build()
        // .mount(
        //     "/",
        //     StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        // )
        .attach(Cors)
        .mount(
            "/",
            routes![index, all_options, get_todos, post_todo, toggle_completed],
        )
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
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
