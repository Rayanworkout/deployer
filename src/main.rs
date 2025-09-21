#[macro_use]
extern crate rocket;

mod db;
mod models;

// Template engine
use rocket::form::FromForm;
use rocket::http::Status;
use rocket::form::Form;
use rocket_dyn_templates::{Template, context};

// FileServer to serve static files
use rocket::fs::FileServer;
use rocket::response::Redirect;

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

#[post("/create-project", data = "<project_form>")]
fn create_project(project_form: Form<ProjectForm>) -> Result<Redirect, (Status, String)> {
    let pj = project_form.into_inner();

    let project = Project {
        name: pj.name,
        description: pj.description,
        github_url: pj.github_url,
    };

    match insert_project(&project) {
        Ok(_) => Ok(Redirect::to(uri!(index))),
        Err(e) => {
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
