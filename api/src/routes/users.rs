use crate::models::event::Event;
use crate::models::user::User;
use crate::repo::users::UserRepository;
use crate::services::users::UserService;
use crate::services::users_events::UserEventService;
use crate::utils::api_response::{ApiResponse, PaginatedItemsResponse};
use neo4rs::Graph;
use rocket::{Route, State};
use std::sync::Arc;
use crate::utils::pagination::PaginationParams;

pub struct UserController {
    user_service: Arc<UserService>,
    user_event_service: Arc<UserEventService>,
}

impl UserController {
    pub fn new(user_service: Arc<UserService>, user_event_service: Arc<UserEventService>) -> Self {
        Self {
            user_service,
            user_event_service,
        }
    }

    pub fn routes() -> Vec<Route> {
        routes![
            get_one,
            get_all,
            get_all_events_of_user,
            recommend_events_for_user_based_on_events_similarity,
        ]
    }
}

#[get("/user/<user_name>")]
pub async fn get_one(controller: &State<UserController>, user_name: &str) -> ApiResponse<User> {
    controller.user_service.get_one(user_name).await
}

#[get("/users?<pagination..>")]
pub async fn get_all(pagination: Option<PaginationParams>, controller: &State<UserController>) -> PaginatedItemsResponse<User> {
    controller.user_service.get_all(pagination).await
}

#[get("/user/<user_name>/events")]
pub async fn get_all_events_of_user(
    controller: &State<UserController>,
    user_name: &str,
) -> ApiResponse<Vec<Event>> {
    controller
        .user_event_service
        .find_all_events_of_user(user_name)
        .await
}

#[get("/user/<user_name>/recommendations")]
pub async fn recommend_events_for_user_based_on_events_similarity(
    controller: &State<UserController>,
    user_name: &str,
) -> ApiResponse<Vec<Event>> {
    controller
        .user_event_service
        .recommend_events_for_user_based_on_events_similarity(user_name)
        .await
}
