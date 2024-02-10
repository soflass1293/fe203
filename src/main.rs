#[macro_use]
extern crate rocket;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use std::fs;
mod utils;
mod routes;

#[launch]
fn rocket() -> _ {
    if fs::metadata("src/db.json").is_err() {
        fs::write("src/db.json", "{}").expect("Unable to write file");
    }
    rocket::build().attach(Cors).mount(
        "/",
        routes![
            routes::index,
            all_options,
            routes::get_todos,
            routes::post_todo,
            routes::toggle_completed,
            routes::delete_todo
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
