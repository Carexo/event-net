use std::sync::Arc;
use neo4rs::Graph;
use crate::models::user::User;
use crate::repo::users::{UserRepository};
use crate::utils::api_response::ApiResponse;

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(graph: Arc<Graph>) -> Self {
        Self { repo: UserRepository::new(graph) }
    }
    
    pub async fn get_one(&self, user_name: String) -> ApiResponse<User> {
        match self.repo.get(user_name).await {
            Ok(user) => ApiResponse::success(user, "User found"),
            Err(e) => ApiResponse::message_only(e.to_string(), e.status())
        }
    }
}