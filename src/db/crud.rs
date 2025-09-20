use crate::models::Command;
use crate::models::Project;
use rusqlite::{Connection, Result, ToSql};

use crate::db::{DB_PATH, TABLE_PROJECTS};

fn insert(table_name: &str, fields: &[&str], values: &[&dyn ToSql]) -> Result<()> {
    let conn = Connection::open(DB_PATH)?;

    // Build placeholders like ?1, ?2, ?3
    let placeholders: Vec<String> = (1..=fields.len()).map(|i| format!("?{}", i)).collect();

    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name,
        fields.join(", "),
        placeholders.join(", ")
    );

    conn.execute(&sql, values)?;
    Ok(())
}

pub fn _insert_project(project: &Project) -> Result<()> {
    insert(
        TABLE_PROJECTS,
        &["name", "description", "github_url"],
        &[&project.name, &project.description, &project.github_url],
    )?;

    println!("Successfully created {} ...", project.name);
    Ok(())
}

pub fn _insert_command(command: &Command) -> Result<()> {
    insert(
        TABLE_PROJECTS,
        &["name", "content", "description"],
        &[&command.name, &command.content, &command.description],
    )?;

    println!("Successfully created {} ...", command.name);
    Ok(())
}
