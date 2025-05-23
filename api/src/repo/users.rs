use std::sync::{Arc};
use neo4rs::{query, Graph};
use rocket::http::Status;
use thiserror::Error;
use crate::models::user::User;
use crate::repo::{ApiError, RepoError};
use crate::repo::RepoError::Other;

#[derive(Error, Debug)]
pub enum UserRepoError {
    #[error("repo error: {0}")]
    RepoError(#[from] RepoError),
    #[error("User not found with user name: {0}")]
    UserNotFound(String),
}

impl ApiError for UserRepoError {
    fn status(&self) -> Status {
        match self {
            UserRepoError::RepoError(e) => e.status(),
            UserRepoError::UserNotFound(_) => Status::BadRequest,
        }
    }
}

pub struct UserRepository {
    graph: Arc<Graph>,
}

impl UserRepository {
    pub fn new(graph: Arc<Graph>) -> Self {
        Self { graph }
    }

    pub async fn find_one(&self, user_name: &str) -> Result<User, UserRepoError> {
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
            Ok(None) => Err(UserRepoError::UserNotFound(user_name.to_string())),
            Err(e) => Err(UserRepoError::RepoError(Other(e.to_string()))),
        }
    }

    pub async fn find_all(&self) -> Result<Vec<User>, UserRepoError> {
        let result = self.graph
            .execute(query("MATCH (u:User) RETURN u")).await;
        let mut rows = result.map_err(|e| UserRepoError::RepoError(Other(e.to_string())))?;

        let mut users = Vec::<User>::new();

        while let Some(row) = match rows.next().await {
            Ok(r) => r,
            Err(e) => return Err(UserRepoError::RepoError(Other(e.to_string()))),
        } {
            let user: User = row.get("u").map_err(|e| UserRepoError::RepoError(Other(e.to_string())))?;
            users.push(user);
        }

        Ok(users)
    }
}