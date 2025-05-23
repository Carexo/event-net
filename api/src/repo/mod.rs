use rocket::http::Status;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] neo4rs::Error),

    #[error("Error: {0}")]
    Other(String),
}

impl RepoError {
    pub fn status(&self) -> Status {
        match self {
            RepoError::DatabaseError(_) => Status::InternalServerError,
            RepoError::Other(_) => Status::InternalServerError,
        }
    }
}

pub mod events;
pub mod users;