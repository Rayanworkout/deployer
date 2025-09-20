use rusqlite::{Connection, Result, ToSql, params};

use crate::db::DB_PATH;
use crate::db::TABLE_PROJECTS;
use crate::models::Project;

pub fn project_exists(project: &Project) -> Result<bool> {
    let conn = Connection::open(DB_PATH)?;

    // Check if any row matches the name OR github_url
    let mut stmt = conn.prepare(&format!(
        "SELECT EXISTS(
            SELECT 1 FROM {} WHERE name = ?1 OR github_url = ?2
        )",
        TABLE_PROJECTS
    ))?;

    let exists: i32 = stmt.query_row(params![&project.name, &project.github_url], |row| {
        row.get(0)
    })?;

    Ok(exists == 1)
}

pub fn insert(table_name: &str, fields: &[&str], values: &[&dyn ToSql]) -> Result<()> {
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
