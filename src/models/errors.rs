#[derive(Debug)]
pub enum ProjectCreationError {
    AlreadyExists,
    // InvalidName,
    // InvalidGithubUrl,
}

use std::fmt;

impl fmt::Display for ProjectCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectCreationError::AlreadyExists => write!(f, "Project already exists"),
            // ProjectCreationError::InvalidName => write!(f, "Invalid project name"),
            // ProjectCreationError::InvalidGithubUrl => write!(f, "Invalid GitHub URL"),
        }
    }
}
