use crate::client::oracle::RdbcOracleConn;
use crate::error::{OrmError, OrmErrorKind, OrmResp};
use crate::pool::RdbcPoolInner;
use crate::{RdbcConn, RdbcConnInner, RdbcDataSource, RdbcPool};
use bb8::Pool;
use bb8_oracle::OracleConnectionManager;
use std::sync::Arc;

pub struct RdbcOraclePool {
    pool: Pool<OracleConnectionManager>,
}

impl RdbcOraclePool {
    pub(crate) async fn get_conn(&self) -> OrmResp<RdbcConnInner> {
        let conn_rs = self.pool.get().await;
        match conn_rs {
            Ok(conn) => {
                let conn = RdbcOracleConn { conn };
                Ok(RdbcConnInner {
                    conn: RdbcConn::Oracle(conn),
                })
            }
            Err(err) => Err(OrmError {
                kind: OrmErrorKind::PoolError,
                msg: err.to_string(),
            }),
        }
    }
}
pub async fn build_oracle_pool(data_source: Arc<RdbcDataSource>) -> OrmResp<RdbcPoolInner> {
    let manager = OracleConnectionManager::new(
        data_source.user.as_str(),
        data_source.password.as_str(),
        data_source.host.as_str(),
    );
    let pool_rs = Pool::builder()
        .max_size(data_source.pool_config.max_size.clone() as u32)
        .build(manager)
        .await;
    match pool_rs {
        Ok(pool) => Ok(RdbcPoolInner {
            datasource: data_source,
            inner: RdbcPool::Oracle(RdbcOraclePool { pool }),
        }),
        Err(err) => Err(OrmError {
            kind: OrmErrorKind::ConnError,
            msg: err.to_string(),
        }),
    }
}
