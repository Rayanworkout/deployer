use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, FromForm)]
pub struct Command {
    pub name: String,
    pub content: String,
    pub description: Option<String>,
}
