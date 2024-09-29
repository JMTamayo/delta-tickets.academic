use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::models::{errors::Exception, event_user_rel::EventUserRel};

pub struct EventsRepository<'p> {
    pool: &'p PgPool,
}

impl<'p> EventsRepository<'p> {
    fn get_pool(&self) -> &'p PgPool {
        self.pool
    }

    pub fn new(pool: &'p PgPool) -> Self {
        EventsRepository { pool }
    }

    pub async fn read_by_id(&self, id: Uuid) -> Result<EventUserRel, Exception> {
        Ok(EventUserRel::from_row(
            &QueryBuilder::<Postgres>::new(format!(
                r#"
				SELECT
					r.id,
					r.event_id,
					r.user_id
				FROM
					events.event_user_relationship r
				WHERE
					r.id = '{}';
			"#,
                id,
            ))
            .build()
            .fetch_one(self.get_pool())
            .await?,
        )?)
    }
}
