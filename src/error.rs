use bb8::RunError;
use bb8_oracle;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use tokio_postgres;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrmError {
    pub kind: OrmErrorKind,
    pub msg: String,
}
impl Display for OrmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("[{}]{}", self.kind.to_string(), self.msg))
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum OrmErrorKind {
    SqlError,
    PoolError,
    DataError,
    ConnError,
    NotSupport,
    NotImplement,
    Other,
}

impl Display for OrmErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            OrmErrorKind::SqlError => "SqlError".to_string(),
            OrmErrorKind::PoolError => "PoolError".to_string(),
            OrmErrorKind::DataError => "DataError".to_string(),
            OrmErrorKind::ConnError => "ConnError".to_string(),
            OrmErrorKind::Other => "Other".to_string(),
            OrmErrorKind::NotSupport => "NotSupport".to_string(),
            OrmErrorKind::NotImplement => "NotImplement".to_string(),
        };
        write!(f, "{}", str)
    }
}

pub type OrmResp<T> = Result<T, OrmError>;

impl From<tokio_postgres::Error> for OrmError {
    fn from(value: tokio_postgres::Error) -> Self {
        OrmError {
            kind: OrmErrorKind::SqlError,
            msg: value.to_string(),
        }
    }
}
impl From<RunError<tokio_postgres::Error>> for OrmError {
    fn from(value: RunError<tokio_postgres::Error>) -> Self {
        OrmError {
            kind: OrmErrorKind::PoolError,
            msg: value.to_string(),
        }
    }
}

impl From<bb8_oracle::oracle::Error> for OrmError {
    fn from(value: bb8_oracle::oracle::Error) -> Self {
        OrmError {
            kind: OrmErrorKind::PoolError,
            msg: value.to_string(),
        }
    }
}
impl From<bb8_oracle::Error> for OrmError {
    fn from(value: bb8_oracle::Error) -> Self {
        OrmError {
            kind: OrmErrorKind::PoolError,
            msg: value.to_string(),
        }
    }
}

impl From<rusqlite::Error> for OrmError {
    fn from(value: rusqlite::Error) -> Self {
        OrmError {
            kind: OrmErrorKind::PoolError,
            msg: value.to_string(),
        }
    }
}
