#[macro_use]
extern crate rocket;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
mod db;
mod helpers;
mod todo;
mod utils;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Cors).mount(
        "/",
        routes![
            all_options,
            routes::index,
            routes::retrieve_todos,
            routes::post_todo,
            routes::patch_todo_completed,
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
