use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use lib_protos::User;

use crate::{
    models::errors::{ErrorKind, Exception},
    services::{auth::AuthService, users::UsersManager},
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
            Router::new().route("/user/", get(verify_ticket)),
        )
    }
}

pub async fn verify_ticket(headers: HeaderMap) -> Response {
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

    let user: User = match UsersManager.get_by_username(username).await {
        Ok(e) => e,
        Err(e) => {
            return e.to_http_error_response().into_response();
        }
    };

    (StatusCode::OK, Json(user)).into_response()
}
