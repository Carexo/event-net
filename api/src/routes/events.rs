use crate::models::event::Event;
use neo4rs::{Graph, query};
use rocket::{Response, State};
use std::sync::Arc;

use crate::repo::events::EventRepository;
use crate::services::events::EventService;
use crate::utils::api_response::ApiResponse;
use rocket::serde::{Serialize, json::Json};

pub struct EventController {
    service: EventService,
}

impl EventController {
    pub fn new(graph: Arc<Graph>) -> Self {
        let repo = EventRepository::new(graph);
        let service = EventService::new(repo);
        Self {service}
    }

    pub fn routes() -> Vec<rocket::Route> {
        routes![get_all, get_one]
    }
}

#[get("/events")]
async fn get_all(controller: &State<EventController>) -> ApiResponse<Vec<Event>> {
    controller.service.get_events().await
}

#[get("/event/<id>")]
async fn get_one(controller: &State<EventController>, id: u16) -> ApiResponse<Event> {
    controller.service.get_event(id).await
}
