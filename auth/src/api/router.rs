use axum::Router;

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
            Router::new().merge(AuthServices::new().get_router()),
        )
    }
}
