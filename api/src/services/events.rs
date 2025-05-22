use rocket::http::Status;
use rocket::response::status;
use crate::models::event::Event;
use crate::repo::events::{EventRepository, RepoError};
use crate::utils::api_response::ApiResponse;

pub struct EventService {
    repo: EventRepository,
}

impl EventService {
    pub fn new(repo: EventRepository) -> Self {
        Self { repo }
    }

    pub async fn get_event(&self, id: u16) -> ApiResponse<Event> {
        match self.repo.find_by_id(id).await {
            Ok(event) => ApiResponse::success(event, "Event found successfully"),
            Err(RepoError::NotFound(_)) => ApiResponse::message_only(format!("No event found with ID: {}", id), Status::NotFound),
            Err(e) => ApiResponse::message_only(format!("{}", e), Status::BadRequest),
        }
    }

    pub async fn get_events(&self) -> ApiResponse<Vec<Event>> {
        match self.repo.find_all().await {
            Ok(events) => ApiResponse::success(events, "Events found successfully"),
            Err(e) => ApiResponse::message_only(format!("{}", e), Status::BadRequest)
        }
    }

    pub async fn add_event(&self, event: Event) -> ApiResponse<Event> {
        match self.repo.add(event).await {
            Ok(event) => ApiResponse::success(event, "Events successfully created"),
            Err(e) => ApiResponse::message_only(format!("{}", e), Status::BadRequest)
        }
    }

    pub async fn remove_event(&self, id: u16) -> ApiResponse<String> {
        match self.repo.remove(id).await {
            Ok(message) => ApiResponse::message_only(message, Status::Ok),
            Err(e) => ApiResponse::message_only(format!("{}", e), Status::BadRequest)
        }
    }
}
