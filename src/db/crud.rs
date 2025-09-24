use crate::models::Command;
use crate::models::Project;
use crate::models::errors::CommandCreationError;
use rusqlite::Connection;
use rusqlite::Result;

use std::io;

use crate::db::utils::{command_exists, delete, insert, project_exists};
use crate::db::{DB_PATH, TABLE_COMMANDS, TABLE_PROJECTS};

use crate::models::ProjectCreationError;

/////////// PROJECTS ///////////

pub fn create_project(project: &Project) -> Result<(), ProjectCreationError> {
    if project_exists(&project)? {
        return Err(ProjectCreationError::AlreadyExists);
    }

    insert(
        TABLE_PROJECTS,
        &["name", "description", "github_url"],
        &[&project.name, &project.description, &project.github_url],
    )?;

    println!("Successfully created the project \"{}\" ...", project.name);
    Ok(())
}

pub fn delete_project(name: &str) -> Result<(), io::Error> {
    delete(TABLE_PROJECTS, "name", name)
}

pub fn get_projects() -> Result<Vec<Project>, ProjectCreationError> {
    let conn = Connection::open(DB_PATH)?;

    let mut stmt = conn.prepare(&format!(
        "SELECT name, description, github_url FROM {}",
        crate::db::TABLE_PROJECTS
    ))?;

    let rows = stmt.query_map([], |row| {
        Ok(Project {
            name: row.get(0)?,
            description: row.get(1)?, // Option<String> works: NULL → None
            github_url: row.get(2)?,
        })
    })?;

    let mut projects = Vec::new();
    for row in rows {
        projects.push(row?);
    }

    Ok(projects)
}

/////////// COMMANDS ///////////
pub fn insert_command(command: &Command) -> Result<(), CommandCreationError> {
    if command_exists(&command.name)? {
        return Err(CommandCreationError::AlreadyExists);
    }

    insert(
        TABLE_COMMANDS,
        &["name", "content", "description"],
        &[&command.name, &command.content, &command.description],
    )?;

    println!(
        "Successfully created the following command: \"{}\" ...",
        command.name
    );
    Ok(())
}

pub fn get_commands() -> Result<Vec<Command>> {
    let conn = Connection::open(DB_PATH)?;

    let mut stmt = conn.prepare(&format!(
        "SELECT name, content, description FROM {}",
        crate::db::TABLE_COMMANDS
    ))?;

    let rows = stmt.query_map([], |row| {
        Ok(Command {
            name: row.get(0)?,
            content: row.get(1)?,
            description: row.get(2)?, // Option<String> works: NULL → None
        })
    })?;

    let mut commands = Vec::new();
    for row in rows {
        commands.push(row?);
    }

    Ok(commands)
}

pub fn delete_command(name: &str) -> Result<(), io::Error> {
    delete(TABLE_COMMANDS, "name", name)
}
