use crate::db::get_projects;
use rocket_dyn_templates::{Template, context};

#[get("/")]
pub fn home() -> Template {
    let projects = get_projects().unwrap_or_else(|_| vec![]);

    Template::render("index", context! { projects: projects })
}