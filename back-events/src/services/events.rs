use lib_protos::{
    user_event_rel_manager_services_client::UserEventRelManagerServicesClient,
    GetUserEventRelRequest, UserEventRel,
};
use tonic::Request;
use uuid::Uuid;

use crate::{
    config::conf::CONFIG,
    models::errors::{ErrorKind, Exception},
};

pub struct EventsManager;

impl EventsManager {
    pub async fn verify_ticket(&self, id: Uuid) -> Result<UserEventRel, Exception> {
        let mut client = match UserEventRelManagerServicesClient::connect(
            CONFIG.get_events_manager_config().get_conn_str(),
        )
        .await
        {
            Ok(client) => client,
            Err(e) => {
                return Err(Exception::new(
                    ErrorKind::InternalServerError,
                    &format!(
                        "Failed to connect to the events manager service**: {}",
                        e.to_string()
                    ),
                ))
            }
        };

        let rel: UserEventRel = match client
            .get(Request::new(GetUserEventRelRequest { id: id.to_string() }))
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
