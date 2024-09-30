use axum::serve as axum_serve;
use tokio::net::TcpListener;

mod api;
mod config;
mod models;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use api::router::ApiRouter;
    use config::{conf::CONFIG, logconfig::LoggingConfig};

    LoggingConfig.init();

    Ok(axum_serve(
        TcpListener::bind(CONFIG.get_server_config().get_socket_address()).await?,
        ApiRouter::new().get_router(),
    )
    .await?)
}
