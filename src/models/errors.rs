#[allow(dead_code)]
#[derive(Debug)]
pub enum ProjectCreationError {
    AlreadyExists,
    InvalidName,
    InvalidGithubUrl,
    DatabaseError(rusqlite::Error),
}

use std::fmt;

impl fmt::Display for ProjectCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectCreationError::AlreadyExists => {
                write!(
                    f,
                    "A project with this name or github URL already exists ..."
                )
            }
            ProjectCreationError::InvalidName => write!(f, "Invalid project name"),
            ProjectCreationError::InvalidGithubUrl => write!(f, "Invalid GitHub URL"),
            ProjectCreationError::DatabaseError(_) => write!(f, "Internal database error"),
        }
    }
}

// This implements automatic conversion from `rusqlite::Error` into our
// custom `ProjectCreationError` type.
//
// Why? Many of our functions call SQLite through `rusqlite`, which
// returns `Result<T, rusqlite::Error>`. By adding this `From` impl,
// we can use the `?` operator in functions that return `Result<T, ProjectCreationError>`.
//
// Example:
//     fn insert_project(...) -> Result<(), ProjectCreationError> {
//         let conn = Connection::open(DB_PATH)?; // if this fails, a rusqlite::Error
//                                                // is automatically turned into
//                                                // ProjectCreationError::DatabaseError
//         Ok(())
//     }
//
// Without this conversion, we would have to write `map_err(ProjectCreationError::DatabaseError)`
// manually everywhere we used `?`.
impl From<rusqlite::Error> for ProjectCreationError {
    fn from(err: rusqlite::Error) -> Self {
        ProjectCreationError::DatabaseError(err)
    }
}
