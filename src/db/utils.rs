use rusqlite::{Connection, Result, params};

use crate::db::DB_PATH;
use crate::db::TABLE_PROJECTS;

pub fn _project_exists(name: &str, github_url: &str) -> Result<bool> {
    let conn = Connection::open(DB_PATH)?;

    // Check if any row matches the name OR github_url
    let mut stmt = conn.prepare(&format!(
        "SELECT EXISTS(
            SELECT 1 FROM {} WHERE name = ?1 OR github_url = ?2
        )",
        TABLE_PROJECTS
    ))?;

    let exists: i32 = stmt.query_row(params![name, github_url], |row| row.get(0))?;

    Ok(exists == 1)
}
