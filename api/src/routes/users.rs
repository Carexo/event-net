use std::sync::Arc;
use neo4rs::Graph;
use rocket::{Route, State};
use crate::models::user::User;
use crate::repo::users::UserRepository;
use crate::services::users::UserService;
use crate::utils::api_response::ApiResponse;

pub struct UserController {
    service: UserService
}

impl UserController {
    pub fn new(user_repo: Arc<Graph>) -> Self {
        Self {
            service: UserService::new(user_repo)
        }
    }

    pub fn routes() -> Vec<Route> {
        routes![get_one]
    }
}

#[get("/user/<user_name>")]
pub async fn get_one(controller: &State<UserController>, user_name: String) -> ApiResponse<User> {
    controller.service.get_one(user_name).await
}