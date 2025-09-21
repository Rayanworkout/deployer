#[macro_use]
extern crate rocket;

mod db;
mod models;

use std::io;
// Template engine
use rocket::form::Form;
use rocket::form::FromForm;
use rocket::http::Status;
use rocket_dyn_templates::{Template, context};

// FileServer to serve static files
use rocket::fs::FileServer;
use rocket::response::Redirect;

use crate::db::{create_project, delete_project, get_projects};
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
fn create_project_view(project_form: Form<ProjectForm>) -> Result<Redirect, (Status, String)> {
    let pj = project_form.into_inner();

    let project = Project {
        name: pj.name,
        description: pj.description,
        github_url: pj.github_url,
    };

    match create_project(&project) {
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

#[delete("/delete-project/<name>")]
fn delete_project_view(name: String) -> Result<Redirect, (Status, String)> {
    match delete_project(&name) {
        Ok(_) => Ok(Redirect::to(uri!(index))),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            Err((Status::NotFound, "Project not found".to_string()))
        }
        Err(e) => Err((Status::InternalServerError, format!("Delete failed: {}", e))),
    }
}

#[launch]
fn rocket() -> _ {
    // Before building the app, we make sure
    // the database exists.
    db::create_database().expect("Failed to init database, aborting ...");

    rocket::build()
        .mount(
            "/",
            routes![index, create_project_view, delete_project_view],
        )
        .attach(Template::fairing())
        .mount("/", FileServer::from("static"))
}
