use lib_protos::{
    user_event_rel_manager_services_server::UserEventRelManagerServices,
    GetUserEventRelRequest as GetUserEventRelRequestProto, UserEventRel as UserEventRelProto,
};
use sqlx::PgPool;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::{db::repository::events::EventsRepository, models::event_user_rel::EventUserRel};

pub struct EventUserRelServiceHandler {
    pool: PgPool,
}

impl EventUserRelServiceHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl UserEventRelManagerServices for EventUserRelServiceHandler {
    async fn get(
        &self,
        request: Request<GetUserEventRelRequestProto>,
    ) -> Result<Response<UserEventRelProto>, Status> {
        let id_str: String = request.into_inner().id;
        let id: Uuid = match Uuid::parse_str(&id_str) {
            Ok(id) => id,
            Err(_) => {
                return Err(Status::invalid_argument("Invalid id"));
            }
        };

        let user: EventUserRel = match EventsRepository::new(&self.pool).read_by_id(id).await {
            Ok(user) => user,
            Err(e) => {
                return Err(e.to_grpc_status());
            }
        };

        Ok(Response::new(UserEventRelProto {
            id: user.get_id().to_string(),
            event_id: user.get_event_id().to_string(),
            user_id: user.get_user_id().to_string(),
        }))
    }
}
