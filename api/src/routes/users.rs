use std::sync::Arc;
use neo4rs::Graph;
use rocket::{Route, State};
use crate::models::user::User;
use crate::repo::users::UserRepository;
use crate::services::users::UserService;
use crate::services::users_events::UserEventService;
use crate::utils::api_response::ApiResponse;

pub struct UserController {
    user_service: Arc<UserService>,
    user_event_service: Arc<UserEventService>
}

impl UserController {
    pub fn new(
        user_service: Arc<UserService>,
        user_event_service: Arc<UserEventService>
    ) -> Self {
        Self {
            user_service,
            user_event_service
        }
    }

    pub fn routes() -> Vec<Route> {
        routes![get_one, get_all]
    }
}

#[get("/user/<user_name>")]
pub async fn get_one(controller: &State<UserController>, user_name: &str) -> ApiResponse<User> {
    controller.user_service.get_one(user_name).await
}

#[get("/users")]
pub async fn get_all(controller: &State<UserController>) -> ApiResponse<Vec<User>> {
    controller.user_service.get_all().await
}