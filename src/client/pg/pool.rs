use crate::client::pg::conn::RdbcPostgresConn;
use crate::client::RdbcPostgresTransaction;
use crate::error::{OrmError, OrmErrorKind, OrmResp};
use crate::{RdbcConn, RdbcDataSource, RdbcPool, RdbcTransaction};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use std::str::FromStr;
use std::sync::Arc;
use tokio_postgres::{Config, NoTls};

pub struct RdbcPostgresPool {
    datasource: Arc<RdbcDataSource>,
    pool: Pool<PostgresConnectionManager<NoTls>>,
}
impl RdbcPostgresPool {
    pub(crate) async fn get_conn(&self) -> OrmResp<RdbcConn> {
        let conn_rs = self.pool.get().await;
        match conn_rs {
            Ok(conn) => {
                let conn = RdbcPostgresConn { conn };
                Ok(RdbcConn::Postgres(conn))
            }
            Err(err) => Err(OrmError {
                kind: OrmErrorKind::PoolError,
                msg: err.to_string(),
            }),
        }
    }
}

pub async fn build_postgres_pool(data_source: Arc<RdbcDataSource>) -> OrmResp<RdbcPool> {
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
                Ok(pool) => Ok(RdbcPool::Postgres(RdbcPostgresPool {
                    datasource: data_source.clone(),
                    pool,
                })),
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
