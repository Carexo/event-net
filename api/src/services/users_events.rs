use std::sync::Arc;
use rocket::http::Status;
use crate::repo::{ApiError};
use crate::repo::users_events::UserEventRepository;
use crate::services::events::EventService;
use crate::services::users::UserService;
use crate::utils::api_response::ApiResponse;
use crate::utils::api_response::ApiResponse::{MessageOnly, Success};

pub struct UserEventService {
    user_service: Arc<UserService>,
    event_service: Arc<EventService>,
    user_event_repo: UserEventRepository
}

impl UserEventService {
    pub fn new(
        user_service: Arc<UserService>,
        event_service: Arc<EventService>,
        user_event_repo: UserEventRepository
    ) -> Self {
        Self { user_service, event_service, user_event_repo }
    }
    
    pub async fn assign_user_to_event(
        &self,
        user_name: &str,
        event_id: u16
    ) -> ApiResponse<String> {
        match self.user_service.get_one(user_name).await {
            Success {data: _, message: _, status: _ } => {},
            MessageOnly {message: m, status: s} => return ApiResponse::message_only(m, s),
        };
        match self.event_service.get_event(event_id).await {
            Success {data: _, message: _, status: _} => {},
            MessageOnly {message: m, status: s} => return ApiResponse::message_only(m, s),
        };
        match self.user_event_repo.assign_user_to_event(user_name, event_id).await {
            Ok(_) => ApiResponse::message_only("User has been assigned to event".to_string(), Status::Created),
            Err(e) => ApiResponse::message_only(e.to_string(), e.status()),
        }
    }
}