use crate::client::RdbcSqliteConn;
use crate::error::{OrmError, OrmErrorKind, OrmResp};
use crate::{RdbcConn, RdbcDataSource, RdbcPool, RdbcTransaction};
use bb8::Pool;
use bb8_sqlite::RusqliteConnectionManager;
use std::sync::Arc;

pub struct RdbcSqlitePool {
    datasource: Arc<RdbcDataSource>,
    pool: Pool<RusqliteConnectionManager>,
}
impl RdbcSqlitePool {
    pub(crate) async fn get_conn(&self) -> OrmResp<RdbcConn> {
        let conn_rs = self.pool.get().await;
        match conn_rs {
            Ok(conn) => {
                let conn = RdbcSqliteConn { conn };
                Ok(RdbcConn::Sqlite(conn))
            }
            Err(err) => Err(OrmError {
                kind: OrmErrorKind::PoolError,
                msg: err.to_string(),
            }),
        }
    }
}

pub async fn build_sqlite_pool(data_source: Arc<RdbcDataSource>) -> OrmResp<RdbcPool> {
    let manage = RusqliteConnectionManager::new("bmbp_msg.db");
    let pool_rs = Pool::builder()
        .max_size(data_source.pool_config.max_size.clone() as u32)
        .build(manage)
        .await;
    match pool_rs {
        Ok(pool) => Ok(RdbcPool::Sqlite(RdbcSqlitePool {
            pool,
            datasource: data_source.clone(),
        })),
        Err(err) => Err(OrmError {
            kind: OrmErrorKind::ConnError,
            msg: err.to_string(),
        }),
    }
}
