use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use lib_protos::UserEventRel;
use uuid::Uuid;

use crate::{
    models::errors::{ErrorKind, Exception},
    services::{auth::AuthService, events::EventsManager},
};

pub struct BackEventsServices {
    path_base: String,
}

impl BackEventsServices {
    pub fn new() -> Self {
        Self {
            path_base: "/".to_string(),
        }
    }

    pub fn get_path_base(&self) -> &str {
        &self.path_base
    }

    pub fn get_router(&self) -> Router {
        Router::new().nest(
            self.get_path_base(),
            Router::new().route("/verify-ticket/:id", get(verify_ticket)),
        )
    }
}

pub async fn verify_ticket(headers: HeaderMap, Path(id): Path<Uuid>) -> Response {
    let username_header = headers.get("username");
    let key_header = headers.get("key");

    let (username, key) = if let (Some(username), Some(key)) = (username_header, key_header) {
        let username = match username.to_str() {
            Ok(u) => u,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(Exception::new(
                        ErrorKind::BadRequest,
                        "Invalid headers in request",
                    )),
                )
                    .into_response()
            }
        };

        let key = match key.to_str() {
            Ok(k) => k,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(Exception::new(
                        ErrorKind::BadRequest,
                        "Invalid headers in request",
                    )),
                )
                    .into_response()
            }
        };

        (username, key)
    } else {
        // Handle the case where one or both headers are not present
        return (
            StatusCode::BAD_REQUEST,
            Json(Exception::new(
                ErrorKind::BadRequest,
                "Invalid headers in request",
            )),
        )
            .into_response();
    };

    match AuthService::new().verify_user(username, key).await {
        Ok(_) => {}
        Err(e) => {
            return e.to_http_error_response().into_response();
        }
    };

    let rel: UserEventRel = match EventsManager.verify_ticket(id).await {
        Ok(e) => e,
        Err(e) => {
            return e.to_http_error_response().into_response();
        }
    };

    (StatusCode::OK, Json(rel)).into_response()
}
