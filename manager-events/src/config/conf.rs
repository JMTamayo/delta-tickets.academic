use std::{
    env::var as from_env,
    net::{Ipv4Addr, SocketAddr},
    num::ParseIntError,
    sync::Arc,
};

use lazy_static::lazy_static;

use crate::models::errors::{ErrorKind, Exception};

#[derive(Debug)]
pub struct DatabaseConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: String,
    max_pool_db_connections: u32,
    migrations_path: String,
}

impl DatabaseConfig {
    pub fn get_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }

    pub fn get_max_pool_db_connections(&self) -> u32 {
        self.max_pool_db_connections
    }

    pub fn get_migrations_path(&self) -> &str {
        &self.migrations_path
    }
}

#[derive(Debug)]
pub struct ServerConfig {
    host: Ipv4Addr,
    port: u16,
}

impl ServerConfig {
    pub fn get_socket_address(&self) -> SocketAddr {
        SocketAddr::new(self.host.into(), self.port)
    }
}

#[derive(Debug)]
pub struct Config {
    database: DatabaseConfig,
    server: ServerConfig,
}

impl Config {
    pub fn get_database_config(&self) -> &DatabaseConfig {
        &self.database
    }

    pub fn get_server_config(&self) -> &ServerConfig {
        &self.server
    }
}

pub trait BuildFromEnvironment {
    fn from_env() -> Result<Self, Exception>
    where
        Self: Sized;
}

impl BuildFromEnvironment for DatabaseConfig {
    fn from_env() -> Result<Self, Exception> {
        let host: String = from_env("POSTGRES_HOST")
            .map_err(|e| Exception::new(ErrorKind::EnvVar, &format!("POSTGRES_PORT- {e}",)))?;

        let port: u16 = from_env("POSTGRES_PORT")
            .map_err(|e| Exception::new(ErrorKind::EnvVar, &format!("POSTGRES_PORT- {e}",)))?
            .parse::<u16>()
            .map_err(|e: ParseIntError| {
                Exception::new(ErrorKind::Parsing, &format!("POSTGRES_PORT- {e}",))
            })?;

        let user: String = from_env("POSTGRES_USER")
            .map_err(|e| Exception::new(ErrorKind::EnvVar, &format!("POSTGRES_USER- {e}",)))?;

        let password: String = from_env("POSTGRES_PASSWORD")
            .map_err(|e| Exception::new(ErrorKind::EnvVar, &format!("POSTGRES_PASSWORD- {e}")))?;

        let database: String = from_env("POSTGRES_DB")
            .map_err(|e| Exception::new(ErrorKind::EnvVar, &format!("POSTGRES_DB- {e}")))?;

        let max_pool_db_connections: u32 = from_env("POSTGRES_MAX_POOL_DB_CONNECTIONS")
            .map_err(|e| {
                Exception::new(
                    ErrorKind::EnvVar,
                    &format!("POSTGRES_MAX_POOL_DB_CONNECTIONS- {e}"),
                )
            })?
            .parse::<u32>()
            .map_err(|e: ParseIntError| {
                Exception::new(
                    ErrorKind::Parsing,
                    &format!("POSTGRES_MAX_POOL_DB_CONNECTIONS- {e}",),
                )
            })?;

        let migrations_path: String = String::from("src/db/migrations");

        Ok(DatabaseConfig {
            host,
            port,
            user,
            password,
            database,
            max_pool_db_connections,
            migrations_path,
        })
    }
}

impl BuildFromEnvironment for ServerConfig {
    fn from_env() -> Result<Self, Exception> {
        let host: Ipv4Addr = Ipv4Addr::UNSPECIFIED;

        let port: u16 = 80;

        Ok(ServerConfig { host, port })
    }
}

impl BuildFromEnvironment for Config {
    fn from_env() -> Result<Self, Exception> {
        let database: DatabaseConfig = DatabaseConfig::from_env()?;

        let server: ServerConfig = ServerConfig::from_env()?;

        Ok(Config { database, server })
    }
}

lazy_static! {
    pub static ref CONFIG: Arc<Config> = Arc::new(Config::from_env().unwrap_or_else(|e| {
        panic!("Failed to load microservice configuration: {e}");
    }));
}
