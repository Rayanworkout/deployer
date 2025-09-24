// https://www.w3resource.com/sqlite/snippets/rust-sqlite.php

use rusqlite::Connection;
use std::fs;

use crate::db::{DB_PATH, TABLE_COMMANDS, TABLE_PROJECTS};

pub fn create_database() -> rusqlite::Result<()> {
    // Ensure db folder exists
    fs::create_dir_all("db").expect("Failed to create");

    let conn = Connection::open(DB_PATH)?;

    // Creating the projects table
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            description TEXT,
            github_url  TEXT
        )",
            TABLE_PROJECTS
        ),
        [],
    )?;

    // And the commands table
    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS {} (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT NOT NULL,
            content    TEXT NOT NULL,
            description  TEXT
        )",
            TABLE_COMMANDS
        ),
        [],
    )?;

    println!("Database and projects table created successfully.");

    Ok(())
}
