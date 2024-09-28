use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Role {
    id: Uuid,
    name: String,
    description: String,
}

impl Role {
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct UserQueryResult {
    id: Uuid,
    username: String,
    password: String,
    role_id: Uuid,
    role_name: String,
    role_description: String,
}

#[derive(Debug, Clone)]
pub struct User {
    id: Uuid,
    username: String,
    password: String,
    role: Role,
}

impl User {
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }

    pub fn get_role(&self) -> &Role {
        &self.role
    }
}

impl From<UserQueryResult> for User {
    fn from(user_query_result: UserQueryResult) -> Self {
        User {
            id: user_query_result.id,
            username: user_query_result.username,
            password: user_query_result.password,
            role: Role {
                id: user_query_result.role_id,
                name: user_query_result.role_name,
                description: user_query_result.role_description,
            },
        }
    }
}
