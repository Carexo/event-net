use crate::models::event::{Event, EventUpdate};
use neo4rs::Graph;
use rocket::State;
use std::sync::Arc;

use crate::repo::events::EventRepository;
use crate::services::events::EventService;
use crate::utils::api_response::ApiResponse;
use rocket::serde::{json::Json};
use crate::services::users_events::UserEventService;

pub struct EventController {
    event_service: Arc<EventService>,
    user_event_service: Arc<UserEventService>
}

impl EventController {
    pub fn new(
        event_service: Arc<EventService>,
        user_event_service: Arc<UserEventService>
    ) -> Self {
        Self { event_service, user_event_service }
    }

    pub fn routes() -> Vec<rocket::Route> {
        routes![get_all, get_one, add, delete, edit, assign_user_to_event]
    }
}

#[get("/events")]
async fn get_all(controller: &State<EventController>) -> ApiResponse<Vec<Event>> {
    controller.event_service.get_events().await
}

#[get("/event/<id>")]
async fn get_one(controller: &State<EventController>, id: u16) -> ApiResponse<Event> {
    controller.event_service.get_event(id).await
}

#[post("/event", format = "application/json", data = "<event>")]
async fn add(controller: &State<EventController>, event: Json<Event>) -> ApiResponse<Event> {
    controller.event_service.add_event(event.into_inner()).await
}

#[delete("/event/<id>")]
async fn delete(controller: &State<EventController>, id: u16) -> ApiResponse<String> {
    controller.event_service.remove_event(id).await
}

#[put("/event/<id>", format = "application/json", data = "<event>")]
async fn edit(controller: &State<EventController>, id: u16, event: Json<EventUpdate>) -> ApiResponse<Event> {
    controller.event_service.edit_event(id, event.into_inner()).await
}

#[put("/events/<event_id>/attendees/<user_name>")]
async fn assign_user_to_event(
    controller: &State<EventController>,
    event_id: u16,
    user_name: &str
) -> ApiResponse<String> {
    controller.user_event_service.assign_user_to_event(user_name, event_id).await
}