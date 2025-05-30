use crate::models::event::{Event, EventUpdate};
use neo4rs::Graph;
use rocket::State;
use std::sync::Arc;

use crate::repo::events::EventRepository;
use crate::services::events::EventService;
use crate::services::users_events::UserEventService;
use crate::utils::api_response::ApiResponse;
use rocket::serde::json::Json;

pub struct EventController {
    event_service: Arc<EventService>,
    user_event_service: Arc<UserEventService>,
}

impl EventController {
    pub fn new(
        event_service: Arc<EventService>,
        user_event_service: Arc<UserEventService>,
    ) -> Self {
        Self {
            event_service,
            user_event_service,
        }
    }

    pub fn routes() -> Vec<rocket::Route> {
        routes![
            get_all,
            get_one,
            add,
            delete,
            edit,
            assign_user_to_event,
            unassign_user_from_event,
            get_featured,
            is_attendees_to_event,
            get_events_by_keywords,
            get_events_keywords
        ]
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
async fn add(controller: &State<EventController>, event: Json<EventUpdate>) -> ApiResponse<Event> {
    controller.event_service.add_event(event.into_inner()).await
}

#[delete("/event/<id>")]
async fn delete(controller: &State<EventController>, id: u16) -> ApiResponse<String> {
    controller.event_service.remove_event(id).await
}

#[put("/event/<id>", format = "application/json", data = "<event>")]
async fn edit(
    controller: &State<EventController>,
    id: u16,
    event: Json<EventUpdate>,
) -> ApiResponse<Event> {
    controller
        .event_service
        .edit_event(id, event.into_inner())
        .await
}

#[put("/events/<event_id>/attendees/<user_name>")]
async fn assign_user_to_event(
    controller: &State<EventController>,
    event_id: u16,
    user_name: &str,
) -> ApiResponse<String> {
    controller
        .user_event_service
        .assign_user_to_event(user_name, event_id)
        .await
}

#[delete("/events/<event_id>/attendees/<user_name>")]
async fn unassign_user_from_event(
    controller: &State<EventController>,
    event_id: u16,
    user_name: &str,
) -> ApiResponse<String> {
    controller
        .user_event_service
        .unassign_user_from_event(user_name, event_id)
        .await
}

#[get("/events/<event_id>/attendees/<user_name>")]
async fn is_attendees_to_event(
    controller: &State<EventController>,
    user_name: &str,
    event_id: u16,
) -> ApiResponse<bool> {
    controller
        .user_event_service
        .is_user_registered_to_event(user_name, event_id)
        .await
}

#[get("/events/featured")]
async fn get_featured(controller: &State<EventController>) -> ApiResponse<Vec<Event>> {
    controller.event_service.get_featured_events().await
}

#[get("/events/filter?<keyword>")]
async fn get_events_by_keywords(controller: &State<EventController>, keyword: Vec<String>) -> ApiResponse<Vec<Event>> {
    controller.event_service.get_events_by_keywords(keyword).await
}

#[get("/events/keywords")]
async fn get_events_keywords(controller: &State<EventController>) -> ApiResponse<Vec<String>> {
    controller.event_service.get_events_keywords().await
}