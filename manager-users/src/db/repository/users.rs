use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};

use crate::models::{
    errors::Exception,
    users::{User, UserQueryResult},
};

pub struct UsersRepository<'p> {
    pool: &'p PgPool,
}

impl<'p> UsersRepository<'p> {
    fn get_pool(&self) -> &'p PgPool {
        self.pool
    }

    pub fn new(pool: &'p PgPool) -> Self {
        UsersRepository { pool }
    }

    pub async fn read_by_username(&self, username: &str) -> Result<User, Exception> {
        Ok(User::from(UserQueryResult::from_row(
            &QueryBuilder::<Postgres>::new(format!(
                r#"
				SELECT
					u.id,
					u.username,
					u.password,
					r.id AS role_id,
					r.name AS role_name,
					r.description AS role_description
				FROM
					app.users u
					INNER JOIN app.roles r ON u.rol_id = r.id
				WHERE
					u.username = '{}';
			"#,
                username,
            ))
            .build()
            .fetch_one(self.get_pool())
            .await?,
        )?))
    }
}
