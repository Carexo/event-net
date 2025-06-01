use crate::models::user::User;
use crate::repo::RepoError::Other;
use crate::repo::{ApiError, RepoError};
use neo4rs::{Graph, query};
use rocket::http::Status;
use std::sync::Arc;
use thiserror::Error;

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
        let result = self
            .graph
            .execute(
                query("MATCH (u:User) WHERE u.name = $name RETURN u")
                    .param("name", user_name.clone()),
            )
            .await;

        let mut rows = result.map_err(|e| UserRepoError::RepoError(Other(e.to_string())))?;

        match rows.next().await {
            Ok(Some(row)) => row
                .get("u")
                .map_err(|e| UserRepoError::RepoError(Other(e.to_string()))),
            Ok(None) => Err(UserRepoError::UserNotFound(user_name.to_string())),
            Err(e) => Err(UserRepoError::RepoError(Other(e.to_string()))),
        }
    }

    pub async fn find_all(&self, page: u32, limit: u32) -> Result<(Vec<User>, u32), UserRepoError> {
        let skip = (page - 1) * limit;

        // let result = self.graph.execute(query("MATCH (u:User) RETURN u")).await;
        let result = self
            .graph
            .execute(
                query(
                    "MATCH (u:User)
                      WITH count(u) AS total
                      MATCH (u:User)
                      RETURN u, total
                      ORDER BY u.name
                      SKIP $skip LIMIT $limit",
                )
                .param("skip", skip as i64)
                .param("limit", limit as i64),
            )
            .await
            .map_err(|e| UserRepoError::RepoError(Other(e.to_string())))?;

        let mut users = Vec::<User>::new();

        let mut rows = result;
        let mut total = 0;

        while let Some(row) = rows.next().await.map_err(|e| UserRepoError::RepoError(Other(e.to_string())))? {
            let user: User = row.get("u").map_err(|e| UserRepoError::RepoError(Other(e.to_string())))?;
            total = row.get::<i64>("total").unwrap_or(0) as u32;
            users.push(user);
        }

        Ok((users, total))
    }
}
