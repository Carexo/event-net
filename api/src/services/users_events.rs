use crate::models::event::Event;
use crate::repo::ApiError;
use crate::repo::users_events::UserEventRepository;
use crate::services::events::EventService;
use crate::services::users::UserService;
use crate::utils::api_response::ApiResponse;
use crate::utils::api_response::ApiResponse::{MessageOnly, Success};
use rocket::http::Status;
use std::sync::Arc;

pub struct UserEventService {
    user_service: Arc<UserService>,
    event_service: Arc<EventService>,
    user_event_repo: UserEventRepository,
}

impl UserEventService {
    pub fn new(
        user_service: Arc<UserService>,
        event_service: Arc<EventService>,
        user_event_repo: UserEventRepository,
    ) -> Self {
        Self {
            user_service,
            event_service,
            user_event_repo,
        }
    }

    pub async fn assign_user_to_event(
        &self,
        user_name: &str,
        event_id: u16,
    ) -> ApiResponse<String> {
        match self.user_service.get_one(user_name).await {
            MessageOnly {
                message: m,
                status: s,
            } => return ApiResponse::message_only(m, s),
            _ => {}
        };

        match self.event_service.get_event(event_id).await {
            MessageOnly {
                message: m,
                status: s,
            } => return ApiResponse::message_only(m, s),
            _ => {}
        };

        match self
            .user_event_repo
            .assign_user_to_event(user_name, event_id)
            .await
        {
            Ok(_) => ApiResponse::message_only(
                "User has been assigned to event".to_string(),
                Status::Created,
            ),
            Err(e) => ApiResponse::message_only(e.to_string(), e.status()),
        }
    }

    pub async fn unassign_user_from_event(
        &self,
        user_name: &str,
        event_id: u16,
    ) -> ApiResponse<String> {
        match self.user_service.get_one(user_name).await {
            MessageOnly {
                message: m,
                status: s,
            } => return ApiResponse::message_only(m, s),
            _ => {}
        };

        match self.event_service.get_event(event_id).await {
            MessageOnly {
                message: m,
                status: s,
            } => return ApiResponse::message_only(m, s),
            _ => {}
        };

        match self
            .user_event_repo
            .unassign_user_from_event(user_name, event_id)
            .await
        {
            Ok(_) => ApiResponse::message_only(
                "User has been unassigned from event".to_string(),
                Status::Ok,
            ),
            Err(e) => ApiResponse::message_only(e.to_string(), e.status()),
        }
    }

    pub async fn find_all_events_of_user(&self, user_name: &str) -> ApiResponse<Vec<Event>> {
        match self.user_service.get_one(user_name).await {
            MessageOnly {
                message: m,
                status: s,
            } => return ApiResponse::message_only(m, s),
            _ => {}
        };

        match self
            .user_event_repo
            .find_all_events_of_user(user_name)
            .await
        {
            Ok(events) => ApiResponse::success(events, "Events are ready".to_string()),
            Err(e) => ApiResponse::message_only(e.to_string(), e.status()),
        }
    }

    pub async fn recommend_events_for_user_based_on_events_similarity(
        &self,
        user_name: &str,
    ) -> ApiResponse<Vec<Event>> {
        match self.user_service.get_one(user_name).await {
            MessageOnly {
                message: m,
                status: s,
            } => return ApiResponse::message_only(m, s),
            _ => {}
        };

        match self
            .user_event_repo
            .recommend_events_for_user_based_on_events_similarity(user_name)
            .await
        {
            Ok(events) => ApiResponse::success(events, "Events are ready".to_string()),
            Err(e) => ApiResponse::message_only(e.to_string(), e.status()),
        }
    }

    pub async fn is_user_registered_to_event(
        &self,
        user_name: &str,
        event_id: u16,
    ) -> ApiResponse<bool> {
        match self.user_service.get_one(user_name).await {
            MessageOnly {
                message: m,
                status: s,
            } => return ApiResponse::message_only(m, s),
            _ => {}
        }

        match self
            .user_event_repo
            .is_user_registered_to_event(user_name, event_id)
            .await
        {
            Ok(is_registered) => ApiResponse::success(is_registered, "User is registered to event"),
            Err(e) => ApiResponse::message_only(e.to_string(), e.status()),
        }
    }
}
