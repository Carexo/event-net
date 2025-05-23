use crate::models::user::User;
use crate::repo::ApiError;
use crate::repo::users::{UserRepository};
use crate::utils::api_response::ApiResponse;

pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }
    
    pub async fn get_one(&self, user_name: &str) -> ApiResponse<User> {
        match self.user_repo.find_one(user_name).await {
            Ok(user) => ApiResponse::success(user, "User found"),
            Err(e) => ApiResponse::message_only(e.to_string(), e.status())
        }
    }
    
    pub async fn get_all(&self) -> ApiResponse<Vec<User>> {
        match self.user_repo.find_all().await {
            Ok(users) => ApiResponse::success(users, "Users are ready"),
            Err(e) => ApiResponse::message_only(e.to_string(), e.status())
        }
    }
}