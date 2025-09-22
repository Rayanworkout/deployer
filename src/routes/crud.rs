use crate::db::{create_project, delete_project};
use crate::models::Project;

use crate::routes;

use std::io;
// Template engine
use rocket::form::Form;
use rocket::form::FromForm;
use rocket::http::Status;
use rocket::response::{Flash, Redirect};

#[derive(FromForm)]
pub struct ProjectForm {
    pub name: String,
    pub description: Option<String>,
    pub github_url: Option<String>,
}

#[post("/create-project", data = "<project_form>")]
pub fn create_project_view(project_form: Form<ProjectForm>) -> Result<Redirect, Flash<Redirect>> {
    let pj = project_form.into_inner();

    let project = Project {
        name: pj.name,
        description: pj.description,
        github_url: pj.github_url,
    };

    match create_project(&project) {
        Ok(_) => Ok(Redirect::to(uri!(routes::home))),
        Err(e) => {
            let msg = e.to_string();
            Err(Flash::error(Redirect::to(uri!(crate::routes::home)), msg))
        }
    }
}

#[delete("/delete-project/<name>")]
pub fn delete_project_view(name: String) -> Result<Redirect, (Status, String)> {
    match delete_project(&name) {
        Ok(_) => Ok(Redirect::to(uri!(routes::home))),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            Err((Status::NotFound, "Project not found".to_string()))
        }
        Err(e) => Err((Status::InternalServerError, format!("Delete failed: {}", e))),
    }
}
