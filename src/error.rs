use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrmError {
    pub kind: OrmErrorKind,
    pub msg: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum OrmErrorKind {
    SqlError,
    PoolError,
    DataError,
    ConnError,
    Other,
}
pub type OrmResp<T> = Result<T, OrmError>;

impl From<r2d2::Error> for OrmError {
    fn from(error: r2d2::Error) -> Self {
        OrmError {
            kind: OrmErrorKind::PoolError,
            msg: error.to_string(),
        }
    }
}

impl From<tokio_postgres::Error> for OrmError {
    fn from(error: tokio_postgres::Error) -> Self {
        OrmError {
            kind: OrmErrorKind::ConnError,
            msg: error.to_string(),
        }
    }
}
