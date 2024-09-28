use lib_protos::{
    users_manager_services_server::UsersManagerServices, GetUserRequest as GetUserRequestProto,
    Role as RoleProto, User as UserProto,
};
use sqlx::PgPool;
use tonic::{Request, Response, Status};

use crate::{db::repository::users::UsersRepository, models::users::User};

pub struct UsersServiceHandler {
    pool: PgPool,
}

impl UsersServiceHandler {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl UsersManagerServices for UsersServiceHandler {
    async fn get(
        &self,
        request: Request<GetUserRequestProto>,
    ) -> Result<Response<UserProto>, Status> {
        let username: String = request.into_inner().username;

        let user: User = match UsersRepository::new(&self.pool)
            .read_by_username(&username)
            .await
        {
            Ok(user) => user,
            Err(e) => {
                return Err(e.to_grpc_status());
            }
        };

        Ok(Response::new(UserProto {
            id: user.get_id().to_string(),
            username: user.get_username().to_string(),
            password: user.get_password().to_string(),
            role: Some(RoleProto {
                id: user.get_role().get_id().to_string(),
                name: user.get_role().get_name().to_string(),
                description: user.get_role().get_description().to_string(),
            }),
        }))
    }
}
