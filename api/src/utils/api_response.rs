use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success { data: T, message: String, #[serde(skip)] status: Status },
    MessageOnly { message: String, #[serde(skip)] status: Status },
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T, message: impl Into<String>) -> Self {
        Self::Success {
            data,
            message: message.into(),
            status: Status::Ok
        }
    }

    pub fn message_only(message: impl Into<String>, status: Status) -> Self {
        Self::MessageOnly {
            message: message.into(),
            status
        }
    }
}

impl<'r, T: Serialize> Responder<'r, 'static> for ApiResponse<T> {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let status = match &self {
            Self::Success { status, .. } => *status,
            Self::MessageOnly { status, .. } => *status,
        };

        let json_response = Json(self);
        rocket::Response::build_from(json_response.respond_to(req)?)
            .status(status)
            .ok()
    }
}
