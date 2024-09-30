use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};

use crate::{
    models::errors::{ErrorKind, Exception},
    services::users::UsersManager,
};

pub struct AuthServices {
    path_base: String,
}

impl AuthServices {
    pub fn new() -> Self {
        Self {
            path_base: "/auth".to_string(),
        }
    }

    pub fn get_path_base(&self) -> &str {
        &self.path_base
    }

    pub fn get_router(&self) -> Router {
        Router::new()
            .nest(
                self.get_path_base(),
                Router::new().route("/verify", post(verify_token)),
            )
    }
}

pub async fn verify_token(headers: HeaderMap) -> Response {
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
                    .into_response();
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

    let is_verified = match UsersManager::new().verify_user(username, key).await {
        Ok(is_verified) => is_verified,
        Err(e) => return (e.to_http_error_response()).into_response(),
    };

    if is_verified {
        (StatusCode::ACCEPTED).into_response()
    } else {
        (StatusCode::UNAUTHORIZED).into_response()
    }
}
