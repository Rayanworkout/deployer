use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub github_url: Option<String>
    // last_commit
    // status
    // recipe ?
}