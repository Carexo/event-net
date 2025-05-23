#[macro_use] extern crate rocket;
mod db;
mod routes;
mod services;
mod models;
mod utils;
mod repo;

use dotenv::dotenv;
use std::env;
use db::neo4j::Neo4jConnection;
use crate::routes::events::EventController;
use crate::routes::users::UserController;

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

    rocket::build()
        .manage(EventController::new(graph.clone()))
        .manage(UserController::new(graph))
        .mount("/", routes![index])
        .mount("/", EventController::routes())
        .mount("/", UserController::routes())
}
