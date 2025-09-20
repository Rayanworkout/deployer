use crate::models::Command;
use crate::models::Project;
use rusqlite::Result;

use crate::db::TABLE_PROJECTS;
use crate::db::utils::{insert, project_exists};

use crate::models::ProjectCreationError;

pub fn insert_project(project: &Project) -> Result<(), ProjectCreationError> {
    if project_exists(&project)? {
        return Err(ProjectCreationError::AlreadyExists);
    }

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
