use bcrypt::verify;
use lib_protos::{users_manager_services_client::UsersManagerServicesClient, GetUserRequest};
use tonic::Request;

use crate::{
    config::conf::CONFIG,
    models::errors::{ErrorKind, Exception},
};

pub struct UsersManager;

impl UsersManager {
    pub fn new() -> Self {
        UsersManager
    }

    pub async fn verify_user(&self, username: &str, key: &str) -> Result<bool, Exception> {
        let mut client =
            match UsersManagerServicesClient::connect(CONFIG.get_users_manager_config().get_conn_str())
                .await
            {
                Ok(client) => client,
                Err(e) => {
                    return Err(Exception::new(
                        ErrorKind::InternalServerError,
                        &format!("Failed to connect to the users manager service**: {}", e.to_string()),
                    ))
                }
            };

        let password: String = match client
            .get(Request::new(GetUserRequest {
                username: username.to_string(),
            }))
            .await
        {
            Ok(response) => response.into_inner().password,
            Err(e) => match e.code() {
                tonic::Code::NotFound => {
                    return Err(Exception::new(
                        ErrorKind::NotFound,
                        &format!("User not found: {}", e),
                    ));
                }
                _ => {
                    return Err(Exception::new(
                        ErrorKind::InternalServerError,
                        &format!("Failed to verify user: {}", e),
                    ));
                }
            },
        };

        match verify(key, &password) {
            Ok(is_valid) => Ok(is_valid),
            Err(e) => Err(Exception::new(
                ErrorKind::InternalServerError,
                &format!("Failed to verify user: {}", e),
            )),
        }
    }
}
