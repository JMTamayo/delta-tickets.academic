use lib_protos::{users_manager_services_client::UsersManagerServicesClient, GetUserRequest, User};

use crate::{
    config::conf::CONFIG,
    models::errors::{ErrorKind, Exception},
};

pub struct UsersManager;

impl UsersManager {
    pub async fn get_by_username(&self, username: &str) -> Result<User, Exception> {
        let mut client = match UsersManagerServicesClient::connect(
            CONFIG.get_users_manager_config().get_conn_str(),
        )
        .await
        {
            Ok(client) => client,
            Err(e) => {
                return Err(Exception::new(
                    ErrorKind::InternalServerError,
                    &format!(
                        "Failed to connect to the users manager service**: {}",
                        e.to_string()
                    ),
                ))
            }
        };

        let rel: User = match client
            .get(GetUserRequest {
                username: username.to_string(),
            })
            .await
        {
            Ok(response) => response.into_inner(),
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

        Ok(rel)
    }
}
