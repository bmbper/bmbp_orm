use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
        };
        write!(f, "{}", str)
    }
}

pub type OrmResp<T> = Result<T, OrmError>;
