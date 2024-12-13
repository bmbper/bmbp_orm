mod adapter;
mod bean;
mod client;
mod conn;
mod ds;
pub mod error;
mod orm;
mod pool;

use crate::error::{OrmError, OrmErrorKind, OrmResp};
pub use bean::*;
pub use conn::*;
pub use ds::PoolConfig;
pub use ds::RdbcDataSource;
pub use ds::RdbcDbType;
pub use error::*;
pub use orm::RdbcOrm;
pub use pool::RdbcPool;
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};

pub static BMBP_ORM: OnceCell<RwLock<RdbcOrm>> = OnceCell::const_new();

pub async fn init_bmbp_orm(ds: RdbcDataSource) -> OrmResp<()> {
    let orm = RdbcOrm::new(Arc::new(ds)).await?;
    match BMBP_ORM.set(RwLock::new(orm)) {
        Ok(v) => Ok(()),
        Err(err) => Err(OrmError {
            kind: OrmErrorKind::PoolError,
            msg: err.to_string(),
        }),
    }
}
