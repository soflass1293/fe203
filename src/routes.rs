use rocket::form::Form;
use rocket::fs::NamedFile;
use rocket::get;

use crate::helpers::{get_todos, remove_todo, add_todo, toggle_todo, stringify};

#[get("/")]
pub async fn index() -> Result<NamedFile, std::io::Error> {
    NamedFile::open("static/index.html").await
}

#[get("/api/todos")]
pub fn retrieve_todos() -> String {
    format!("{}", stringify(get_todos()))
}

#[derive(FromForm, Debug)]
pub struct UserInput<'r> {
    pub title: &'r str,
}

#[post("/api/todos", data = "<user_input>")]
pub fn post_todo(user_input: Form<UserInput<'_>>) -> String {
    add_todo(user_input.title);
    format!("{}", stringify(get_todos()))
}

#[patch("/api/todos/<id>/completed")]
pub fn patch_todo_completed(id: &str) -> String {
    toggle_todo(id);
    format!("{}", stringify(get_todos()))
}

#[delete("/api/todos/<id>")]
pub fn delete_todo(id: &str) -> String {
    remove_todo(id);
    format!("{}", stringify(get_todos()))
}
