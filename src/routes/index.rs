use crate::db::{get_projects, get_commands};
use rocket::request::FlashMessage;
use rocket_dyn_templates::{Template, context};

#[get("/")]
pub fn home(flash: Option<FlashMessage<'_>>) -> Template {
    let projects = get_projects().unwrap_or_else(|_| vec![]);
    let commands = get_commands().unwrap_or_else(|_| vec![]);

    let message = flash.map(|f| f.message().to_string());

    Template::render("index", context! { commands, projects, message })
}
