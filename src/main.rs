#[macro_use]
extern crate rocket;

mod db;
mod models;

// Template engine
use rocket::form::FromForm;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_dyn_templates::{Template, context};

// FileServer to serve static files
use rocket::fs::FileServer;

use crate::db::{get_projects, insert_project};
use crate::models::{Project, ProjectCreationError};

#[get("/")]
fn index() -> Template {
    let projects = get_projects().unwrap_or_else(|_| vec![]);

    Template::render("index", context! { projects: projects })
}

#[derive(FromForm)]
pub struct ProjectForm {
    pub name: String,
    pub description: Option<String>,
    pub github_url: Option<String>,
}

#[post("/create-project", format = "json", data = "<project_json>")]
fn create_project(project_json: Json<Project>) -> Result<Json<Vec<Project>>, (Status, String)> {
    let pj = project_json.into_inner();

    let project = Project {
        name: pj.name,
        description: pj.description,
        github_url: pj.github_url,
    };

    match insert_project(&project) {
        Ok(_) => {
            // Fetch all projects again
            match get_projects() {
                Ok(projects) => Ok(Json(projects)),
                Err(e) => Err((Status::InternalServerError, e.to_string())),
            }
        }
        Err(e) => {
            // Map your custom error into HTTP status + message
            let (status, msg) = match e {
                ProjectCreationError::AlreadyExists => (Status::Conflict, e.to_string()),
                ProjectCreationError::InvalidName => (Status::BadRequest, e.to_string()),
                ProjectCreationError::InvalidGithubUrl => (Status::BadRequest, e.to_string()),
                ProjectCreationError::DatabaseError(_) => {
                    (Status::InternalServerError, e.to_string())
                }
            };
            Err((status, msg))
        }
    }
}

#[launch]
fn rocket() -> _ {
    // Before building the app, we make sure
    // the database exists.
    db::create_database().expect("Failed to init database, aborting ...");

    rocket::build()
        .mount("/", routes![index, create_project])
        .attach(Template::fairing())
        .mount("/", FileServer::from("static"))
}
