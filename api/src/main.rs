#[macro_use] extern crate rocket;
mod db;
mod routes;
mod services;
mod models;
mod utils;
mod repo;

use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use db::neo4j::Neo4jConnection;
use crate::routes::events::EventController;
use crate::routes::users::UserController;
use crate::repo::events::EventRepository;
use crate::repo::users::UserRepository;
use crate::repo::users_events::UserEventRepository;
use crate::services::events::EventService;
use crate::services::users::UserService;
use crate::services::users_events::UserEventService;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let neo4j_uri = env::var("DB_URI").unwrap_or_else(|_| "bolt://neo4j:7687".to_string());
    let neo4j_user = env::var("DB_USER").unwrap_or_else(|_| "neo4j".to_string());
    let neo4j_password = env::var("DB_PASSWORD").expect("NEO4J_PASSWORD not set");

    let neo4j = Neo4jConnection::new(
        &neo4j_uri,
        &neo4j_user,
        &neo4j_password
    ).await.expect("Failed to connect to Neo4j");

    let graph = neo4j.graph;

    let user_repo = UserRepository::new(graph.clone());
    let event_repo = EventRepository::new(graph.clone());
    let user_event_repo = UserEventRepository::new(graph);

    let user_service = Arc::new(UserService::new(user_repo));
    let event_service = Arc::new(EventService::new(event_repo));
    let user_event_service = Arc::new(UserEventService::new(
        user_service.clone(),
        event_service.clone(),
        user_event_repo
    ));

    let event_controller = EventController::new(
        event_service,
        user_event_service.clone()
    );
    let user_controller = UserController::new(
        user_service,
        user_event_service
    );

    rocket::build()
        .manage(event_controller)
        .manage(user_controller)
        .mount("/", routes![index])
        .mount("/", EventController::routes())
        .mount("/", UserController::routes())
}
