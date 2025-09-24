use crate::db::{delete_command, get_commands, insert_command};
use crate::models::Command;

use rocket::form::Form;
use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::{Template, context};

use std::io;

#[get("/commands")]
pub fn commands_page(flash: Option<FlashMessage<'_>>) -> Template {
    let commands = get_commands().unwrap_or_else(|_| vec![]);
    let message = flash.map(|f| f.message().to_string());

    Template::render("partials/commands_list", context! { commands, message })
}

#[post("/create-command", data = "<command_form>")]
pub fn create_command_endpoint(command_form: Form<Command>) -> Result<Redirect, Flash<Redirect>> {
    let cmd = command_form.into_inner();

    let command = Command {
        name: cmd.name,
        content: cmd.content,
        description: cmd.description,
    };

    match insert_command(&command) {
        Ok(_) => Ok(Redirect::to(uri!(commands_page))),
        Err(e) => {
            let msg = e.to_string();
            Err(Flash::error(Redirect::to(uri!(commands_page)), msg))
        }
    }
}

#[delete("/delete-command/<name>")]
pub fn delete_command_endpoint(name: String) -> Result<Redirect, (Status, String)> {
    match delete_command(&name) {
        Ok(_) => Ok(Redirect::to(uri!(commands_page))),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            Err((Status::NotFound, "Command not found".to_string()))
        }
        Err(e) => Err((Status::InternalServerError, format!("Delete failed: {}", e))),
    }
}
