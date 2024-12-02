mod adapter;
mod conn;
mod ds;
mod error;
mod orm;
mod pool;
mod row;

pub use conn::*;
pub use ds::PoolConfig;
pub use ds::RdbcDataSource;
pub use ds::RdbcDbType;
use once_cell::sync::OnceCell;
pub use orm::RdbcOrm;
pub use pool::RdbcPool;
use tokio::sync::RwLock;
pub static BMBP_ORM: OnceCell<RwLock<RdbcOrm>> = OnceCell::new();
