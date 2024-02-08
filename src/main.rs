
#[macro_use]
extern crate rocket;
use rand::Rng;
use std::collections::HashMap;
use std::iter;
use serde::{Deserialize, Serialize};
use std::fs;

fn id(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}
#[derive(Serialize, Deserialize)]
struct Todo {
    title: String,
    completed: bool,
}
struct Todos {
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

#[get("/todos")]
fn get_todos() -> String {
  let content = fs::read_to_string("src/todos.json")
    .expect("Should have been able to read the file");
  format!("{}", content)
}
#[post("/todos/<title>")]
fn get_todos() -> String {
  let content = fs::read_to_string("src/todos.json")
    .expect("Should have been able to read the file");
  format!("{}", content)
}

#[launch]
fn rocket() -> _ {
    let mut todos = Todos::new();
    rocket::build().mount("/", routes![get_todos])
}
