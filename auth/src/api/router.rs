use axum::{body::Body, http::{Request,StatusCode}, middleware::{self, Next}, response::{IntoResponse, Response}, Router};

use crate::api::auth::AuthServices;

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
            Router::new()
                .merge(AuthServices::new().get_router())
                .route("/auth/verify", axum::routing::options(handler_for_options)),
        )
		.layer(middleware::from_fn(add_cors_header))
    }
}

async fn add_cors_header(req: Request<Body>, next: Next) -> impl IntoResponse {
    let mut response = next.run(req).await;
    response.headers_mut().insert("Access-Control-Allow-Origin", "*".parse().unwrap());
    response.headers_mut().insert("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS".parse().unwrap());
    response.headers_mut().insert("Access-Control-Allow-Headers", "Content-Type, Authorization, username, key".parse().unwrap());
    
    response
}

// OPTIONS handler
async fn handler_for_options() -> impl IntoResponse {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type, Authorization, username, key")
        .body(Body::empty())
        .unwrap();
    response
}