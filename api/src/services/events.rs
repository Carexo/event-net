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
            Err(RepoError::NotFound(_)) => ApiResponse::error(format!("No event found with ID: {}", id)),
            Err(e) => ApiResponse::error(format!("{}", e)),
        }
    }

    pub async fn get_events(&self) -> ApiResponse<Vec<Event>> {
        match self.repo.find_all().await {
            Ok(events) => ApiResponse::success(events, "Events found successfully"),
            Err(e) => ApiResponse::error(format!("{}", e))
        }
    }
}
