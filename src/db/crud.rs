use crate::models::Project;
use rusqlite::{Connection, Result, params};

use crate::db::{DB_PATH, TABLE_PROJECTS};

pub fn insert_project(project: &Project) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    // Insert a new user
    conn.execute(
        &format!(
            "INSERT INTO {} (name, description, github_url) VALUES (?1, ?2, ?3)",
            TABLE_PROJECTS
        ),
        params![project.name, project.description, project.github_url],
    )?;

    println!("Successfully created {} ...", project.name);
    Ok(())
}
