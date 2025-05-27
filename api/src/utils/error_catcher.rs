use rocket::http::Status;
use rocket::Request;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    status: u16,
    message: String
}

#[catch(404)]
fn not_found(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 404,
        message: format!("Resource '{}' not found", req.uri())
    })
}

#[catch(500)]
fn internal_error() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: 500,
        message: "Internal server error occurred".to_string()
    })
}

#[catch(default)]
fn default_catcher(status: Status, req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        status: status.code,
        message: format!("Request to '{}' failed with status: {}", req.uri(), status)
    })
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![not_found, internal_error, default_catcher]
}