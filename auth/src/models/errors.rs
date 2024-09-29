use std::env::VarError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum ErrorKind {
    #[serde(rename = "Environment variable error")]
    EnvVar,
    #[serde(rename = "Bad request")]
    BadRequest,
    #[serde(rename = "Not found")]
    NotFound,
    #[serde(rename = "Internal server error")]
    InternalServerError,
    #[serde(rename = "Parse error")]
    Parsing,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            ErrorKind::EnvVar => write!(f, "Environment variable error"),
            ErrorKind::BadRequest => write!(f, "Bad request"),
            ErrorKind::NotFound => write!(f, "Not found"),
            ErrorKind::InternalServerError => write!(f, "Internal server error"),
            ErrorKind::Parsing => write!(f, "Parse error"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
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

    pub fn to_http_error_response(&self) -> (StatusCode, Json<Exception>) {
        match self.get_kind() {
            ErrorKind::BadRequest => (StatusCode::BAD_REQUEST, Json(self.clone())),
            ErrorKind::NotFound => (StatusCode::NOT_FOUND, Json(self.clone())),
            ErrorKind::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(self.clone()))
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(self.clone())),
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
