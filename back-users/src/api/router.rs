use axum::{body::Body, http::Request, middleware::{self, Next}, response::IntoResponse, Router};

use crate::api::events::BackEventsServices;

pub struct ApiRouter;

impl ApiRouter {
    pub fn new() -> Self {
        Self {}
    }

    fn get_path_base(&self) -> &str {
        "/"
    }

    pub fn get_router(&self) -> Router {
        Router::new().nest(
            self.get_path_base(),
            Router::new().merge(BackEventsServices::new().get_router()),
        )
		.layer(middleware::from_fn(add_cors_header))
    }
}

async fn add_cors_header(req: Request<Body>, next: Next) -> impl IntoResponse {
    let mut response = next.run(req).await;
    response
        .headers_mut()
        .insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    response
}