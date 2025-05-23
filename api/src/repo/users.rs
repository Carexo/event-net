use std::sync::{Arc, Mutex};
use neo4rs::{query, Graph};
use rocket::http::Status;
use thiserror::Error;
use crate::models::user::User;
use crate::repo::events::EventRepository;
use crate::repo::RepoError;
use crate::repo::RepoError::Other;

#[derive(Error, Debug)]
pub enum UserRepoError {
    #[error("repo error: {0}")]
    RepoError(#[from] RepoError),
    #[error("User not found with user name: {0}")]
    UserNotFound(String),
}

impl UserRepoError {
    pub fn status(&self) -> Status {
        match self {
            UserRepoError::RepoError(e) => e.status(),
            UserRepoError::UserNotFound(_) => Status::NotFound,
        }
    }
}

pub struct UserRepository {
    graph: Arc<Graph>,
    event_repo: Option<Arc<Mutex<EventRepository>>>
}

impl UserRepository {
    pub fn new(graph: Arc<Graph>) -> Self {
        Self { graph, event_repo: None }
    }

    pub async fn get(&self, user_name: String) -> Result<User, UserRepoError> {
        let result = self.graph
            .execute(
                query("MATCH (u:User) WHERE u.name = $name RETURN u")
                    .param("name", user_name.clone())
            )
            .await;

        let mut rows = result.map_err(|e| UserRepoError::RepoError(Other(e.to_string())))?;

        match rows.next().await {
            Ok(Some(row)) => {
                row.get("u").map_err(|e| UserRepoError::RepoError(Other(e.to_string())))
            },
            Ok(None) => Err(UserRepoError::UserNotFound(user_name)),
            Err(e) => Err(UserRepoError::RepoError(Other(e.to_string()))),
        }
    }
}