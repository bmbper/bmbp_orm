use crate::client::pg::conn::RdbcPostgresConn;
use crate::error::{OrmError, OrmErrorKind, OrmResp};
use crate::pool::RdbcPoolInner;
use crate::{RdbcConn, RdbcConnInner, RdbcDataSource, RdbcPool};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use std::str::FromStr;
use std::sync::Arc;
use tokio_postgres::{Config, NoTls};

pub struct RdbcPostgresPool {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}
impl RdbcPostgresPool {
    pub(crate) async fn get_conn(&self) -> OrmResp<RdbcConnInner> {
        let conn_rs = self.pool.get().await;
        match conn_rs {
            Ok(conn) => {
                let conn = RdbcPostgresConn { conn };
                Ok(RdbcConnInner {
                    conn: RdbcConn::Postgres(conn),
                })
            }
            Err(err) => Err(OrmError {
                kind: OrmErrorKind::PoolError,
                msg: err.to_string(),
            }),
        }
    }
}

pub async fn build_postgres_pool(data_source: Arc<RdbcDataSource>) -> OrmResp<RdbcPoolInner> {
    let conn_str = format!(
        "host={} port={} user={} password={} dbname={}",
        data_source.host,
        data_source.port,
        data_source.user,
        data_source.password,
        data_source.db_name
    );

    match Config::from_str(conn_str.as_str()) {
        Ok(cf) => {
            let manage = PostgresConnectionManager::new(cf, NoTls);
            let pool_rs = Pool::builder()
                .max_size(data_source.pool_config.max_size.clone() as u32)
                .build(manage)
                .await;
            match pool_rs {
                Ok(pool) => Ok(RdbcPoolInner {
                    datasource: data_source,
                    inner: RdbcPool::Postgres(RdbcPostgresPool { pool }),
                }),
                Err(err) => Err(OrmError {
                    kind: OrmErrorKind::ConnError,
                    msg: err.to_string(),
                }),
            }
        }
        Err(e) => Err(OrmError {
            kind: OrmErrorKind::PoolError,
            msg: e.to_string(),
        }),
    }
}
