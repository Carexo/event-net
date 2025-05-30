use rocket::http::Status;
use crate::models::event::{Event, EventUpdate};
use crate::repo::ApiError;
use crate::repo::events::{EventRepository, EventRepoError};
use crate::utils::api_response::ApiResponse;

pub struct EventService {
    event_repo: EventRepository
}

impl EventService {
    pub fn new(
        event_repo: EventRepository,
    ) -> Self {
        Self { event_repo }
    }

    pub async fn get_event(&self, id: u16) -> ApiResponse<Event> {
        match self.event_repo.find_by_id(id).await {
            Ok(event) => ApiResponse::success(event, "Event found successfully"),
            Err(EventRepoError::NotFound(_)) => ApiResponse::message_only(format!("No event found with ID: {}", id), Status::NotFound),
            Err(e) => ApiResponse::message_only(format!("{}", e), Status::BadRequest),
        }
    }

    pub async fn get_events(&self) -> ApiResponse<Vec<Event>> {
        match self.event_repo.find_all().await {
            Ok(events) => ApiResponse::success(events, "Events found successfully"),
            Err(e) => ApiResponse::message_only(format!("{}", e), Status::BadRequest)
        }
    }

    pub async fn add_event(&self, event: EventUpdate) -> ApiResponse<Event> {
        match self.event_repo.add(event).await {
            Ok(event) => ApiResponse::success(event, "Events successfully created"),
            Err(e) => ApiResponse::message_only(format!("{}", e), Status::BadRequest)
        }
    }

    pub async fn remove_event(&self, id: u16) -> ApiResponse<String> {
        match self.event_repo.remove(id).await {
            Ok(message) => ApiResponse::message_only(message, Status::Ok),
            Err(e) => ApiResponse::message_only(format!("{}", e), Status::BadRequest),
        }
    }

    pub async fn edit_event(&self, id: u16, event: EventUpdate) -> ApiResponse<Event> {
        match self.event_repo.edit(id, event).await {
            Ok(event) => ApiResponse::success(event, "Event edited successfully"),
            Err(e) => ApiResponse::message_only(e.to_string(), e.status())
        }
    }

    pub async fn get_featured_events(&self) -> ApiResponse<Vec<Event>> {
        match self.event_repo.get_featured().await {
            Ok(events) => ApiResponse::success(events, "Events found successfully"),
            Err(e) => ApiResponse::message_only(e.to_string(), e.status())
        }
    }
    
    pub async fn get_events_by_keywords(&self, keyword: Vec<String>) -> ApiResponse<Vec<Event>> {
        match self.event_repo.get_events_by_keywords(keyword).await {
            Ok(events) => ApiResponse::success(events, "Events found successfully"),
            Err(e) => ApiResponse::message_only(e.to_string(), e.status())
        }
    }
}
