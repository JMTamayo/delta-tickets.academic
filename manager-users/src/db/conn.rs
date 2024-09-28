use std::path::Path;

use sqlx::{
    migrate::Migrator,
    postgres::{PgPool, PgPoolOptions},
};

use crate::{config::conf::DatabaseConfig, models::errors::Exception};

pub struct DBHandler<'c>(&'c DatabaseConfig);

impl<'c> DBHandler<'c> {
    pub fn new(config: &'c DatabaseConfig) -> Self {
        Self(config)
    }

    pub async fn get_pool(&self) -> Result<PgPool, Exception> {
        let pool: PgPool = PgPoolOptions::new()
            .max_connections(self.0.get_max_pool_db_connections())
            .connect(&self.0.get_connection_string())
            .await?;

        Ok(pool)
    }

    pub async fn run_migrations(&self, conn_pool: &PgPool) -> Result<(), Exception> {
        let migrations_path: String = self.0.get_migrations_path().to_string();

        Migrator::new(Path::new(&migrations_path))
            .await?
            .run(conn_pool)
            .await?;

        Ok(())
    }
}
