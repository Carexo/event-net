use crate::models::event::Event;
use neo4rs::{Graph};
use rocket::{State};
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
        Self { service }
    }

    pub fn routes() -> Vec<rocket::Route> {
        routes![get_all, get_one, add, delete]
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

#[post("/event", format = "application/json", data = "<event>")]
async fn add(controller: &State<EventController>, event: Json<Event>) -> ApiResponse<Event> {
    controller.service.add_event(event.into_inner()).await
}

#[delete("/event/<id>")]
async fn delete(controller: &State<EventController>, id: u16) -> ApiResponse<String> {
    controller.service.remove_event(id).await
}
