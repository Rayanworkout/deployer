// https://www.w3resource.com/sqlite/snippets/rust-sqlite.php

use rusqlite::Connection;
use std::fs;

pub fn create_database() -> rusqlite::Result<()> {
    // Ensure db folder exists
    fs::create_dir_all("db").expect("Failed to create");

    let conn = Connection::open("db/deployer.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id      INTEGER PRIMARY KEY AUTOINCREMENT,
            name    TEXT NOT NULL,
            description  TEXT NOT NULL,
            github_url  TEXT NOT NULL
        )",
        [],
    )?;

    println!("Database and project table created successfully.");

    Ok(())
}

