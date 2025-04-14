#[macro_use] extern crate rocket;
mod db;
mod routes;
mod services;

use dotenv::dotenv;
use std::env;
use db::neo4j::Neo4jConnection;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    let neo4j_uri = env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://neo4j:7687".to_string());
    let neo4j_user = env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string());
    let neo4j_password = env::var("NEO4J_PASSWORD").expect("NEO4J_PASSWORD not set");

    let neo4j = Neo4jConnection::new(
        &neo4j_uri,
        &neo4j_user,
        &neo4j_password
    ).await.expect("Failed to connect to Neo4j");

    rocket::build()
        .manage(neo4j.graph)
        .mount("/", routes![index, routes::example::example])
}
