use lib_protos::{
    user_event_rel_manager_services_server::UserEventRelManagerServicesServer, FILE_DESCRIPTOR_SET,
};
use std::error::Error as StdError;

use sqlx::PgPool;
use tonic::transport::Server;

mod api;
mod config;
mod db;
mod models;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    use api::users::EventUserRelServiceHandler;
    use config::{conf::CONFIG, logconfig::LoggingConfig};
    use db::conn::DBHandler;

    LoggingConfig.init();

    let db_handler: DBHandler = DBHandler::new(CONFIG.get_database_config());

    tracing::info!(
        "Starting server on {}",
        CONFIG.get_server_config().get_socket_address()
    );
    tracing::info!("Server configuration: {:?}", CONFIG.get_server_config());
    tracing::info!("Databace configuration: {:?}", CONFIG.get_database_config());

    let conn_pool: PgPool = db_handler.get_pool().await?;
    db_handler.run_migrations(&conn_pool).await?;

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1alpha()?;

    Ok(Server::builder()
        .add_service(UserEventRelManagerServicesServer::new(
            EventUserRelServiceHandler::new(conn_pool),
        ))
        .add_service(reflection_service)
        .serve(CONFIG.get_server_config().get_socket_address())
        .await?)
}
