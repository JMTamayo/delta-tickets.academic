use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response,
};

use crate::{
    config::conf::CONFIG,
    models::errors::{ErrorKind, Exception},
};

pub struct AuthService;

impl AuthService {
    pub fn new() -> Self {
        Self
    }

    pub async fn verify_user(&self, username: &str, password: &str) -> Result<(), Exception> {
        let user: HeaderValue = match HeaderValue::from_str(username) {
            Ok(user) => user,
            Err(_) => {
                return Err(Exception::new(
                    ErrorKind::BadRequest,
                    "Error verifying user",
                ))
            }
        };
        let key: HeaderValue = match HeaderValue::from_str(password) {
            Ok(key) => key,
            Err(_) => {
                return Err(Exception::new(
                    ErrorKind::BadRequest,
                    "Error verifying user",
                ))
            }
        };

        let mut headers = HeaderMap::new();
        headers.insert("username", user);
        headers.insert("key", key);

        let url: String = format!(
            "{}/auth/verify",
            CONFIG.get_auth_service_config().get_conn_str()
        );
        let response: Response = match Client::new().post(&url).headers(headers).send().await {
            Ok(response) => response,
            Err(e) => {
                tracing::error!("{:?}", e);
                return Err(Exception::new(
                    ErrorKind::InternalServerError,
                    "Error verifying user",
                ));
            }
        };

        match response.status().is_success() {
            true => Ok(()),
            false => {
                tracing::error!("{:?}", response.status());
                Err(Exception::new(
                    ErrorKind::Unauthorized,
                    "Invalid username or password",
                ))
            }
        }
    }
}
