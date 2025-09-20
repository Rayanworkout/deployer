#[macro_use]
extern crate rocket;

// Template engine
use rocket_dyn_templates::Template;

// FileServer to serve static files
use rocket::fs::FileServer;

use serde_json::json;

#[get("/")]
fn index() -> Template {
    Template::render("index", json!({"test": 2}))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .mount("/", FileServer::from("static"))
}
