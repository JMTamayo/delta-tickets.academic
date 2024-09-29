use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct EventUserRel {
    id: Uuid,
    event_id: Uuid,
    user_id: Uuid,
}

impl EventUserRel {
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_event_id(&self) -> Uuid {
        self.event_id
    }

    pub fn get_user_id(&self) -> Uuid {
        self.user_id
    }
}
