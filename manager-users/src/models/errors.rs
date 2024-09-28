use std::env::VarError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::num::ParseIntError;

use sqlx::{migrate::MigrateError, Error as SqlxError};
use tonic::Status;

#[derive(Debug)]
pub enum ErrorKind {
    EnvVar,
    IO,
    Parsing,
    Database,
    NotFound,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ErrorKind::EnvVar => write!(f, "Environment variable error"),
            ErrorKind::IO => write!(f, "I/O error"),
            ErrorKind::Parsing => write!(f, "Parsing error"),
            ErrorKind::Database => write!(f, "Database error"),
            ErrorKind::NotFound => write!(f, "Not found error"),
        }
    }
}

#[derive(Debug)]
pub struct Exception {
    kind: ErrorKind,
    details: String,
}

impl Exception {
    pub fn get_kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn get_details(&self) -> &str {
        &self.details
    }

    pub fn new(kind: ErrorKind, details: &str) -> Self {
        Exception {
            kind,
            details: details.to_owned(),
        }
    }

    pub fn to_grpc_status(&self) -> Status {
        match self.kind {
            ErrorKind::NotFound => tonic::Status::not_found(self.details.clone()),
            _ => tonic::Status::internal(self.details.clone()),
        }
    }
}

impl Display for Exception {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}: {}", self.get_kind(), self.get_details())
    }
}

impl StdError for Exception {}

impl From<VarError> for Exception {
    fn from(error: VarError) -> Self {
        Exception::new(ErrorKind::EnvVar, &error.to_string())
    }
}

impl From<IoError> for Exception {
    fn from(error: IoError) -> Self {
        Exception::new(ErrorKind::IO, &error.to_string())
    }
}

impl From<ParseIntError> for Exception {
    fn from(error: ParseIntError) -> Self {
        Exception::new(ErrorKind::Parsing, &error.to_string())
    }
}

impl From<SqlxError> for Exception {
    fn from(error: sqlx::Error) -> Self {
        match error {
            SqlxError::RowNotFound => Exception::new(ErrorKind::NotFound, &error.to_string()),
            _ => Exception::new(ErrorKind::Database, &error.to_string()),
        }
    }
}

impl From<MigrateError> for Exception {
    fn from(error: MigrateError) -> Self {
        Exception::new(ErrorKind::Database, &error.to_string())
    }
}
