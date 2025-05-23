use rocket::http::Status;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] neo4rs::Error),

    #[error("Error: {0}")]
    Other(String),
}

pub trait ApiError {
    fn status(&self) -> Status;
}

impl ApiError for RepoError {
    fn status(&self) -> Status {
        match self {
            RepoError::DatabaseError(_) => Status::InternalServerError,
            RepoError::Other(_) => Status::InternalServerError,
        }
    }
}

pub mod events;
pub mod users;
pub mod users_events;