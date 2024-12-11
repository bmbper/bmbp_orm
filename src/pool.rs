use crate::adapter::MySqlConnectionManager;
use crate::bean::RdbcOrmRow;
use crate::ds::{RdbcDataSource, RdbcDbType};
use crate::error::{OrmError, OrmErrorKind, OrmResp};
use crate::{
    RdbcConn, RdbcConnInner, RdbcMysqlConn, RdbcOracleConn, RdbcPostgresConn, RdbcSqliteConn,
};
use bb8::Pool;
use bb8_oracle::OracleConnectionManager;
use bb8_postgres::PostgresConnectionManager;
use bb8_sqlite::RusqliteConnectionManager;
use bmbp_sql::RdbcQueryWrapper;
use mysql_async::Opts;
use std::str::FromStr;
use std::sync::Arc;
use tokio_postgres::{Config, NoTls};

pub enum RdbcPool {
    Sqlite(RdbcSqlitePool),
    Oracle(RdbcOraclePool),
    Mysql(RdbcMysqlPool),
    Postgres(RdbcPostgresPool),
}

impl RdbcPool {
    pub async fn get_conn(&self) -> OrmResp<RdbcConnInner> {
        match self {
            RdbcPool::Sqlite(p) => p.get_conn().await,
            RdbcPool::Oracle(p) => p.get_conn().await,
            RdbcPool::Mysql(p) => p.get_conn().await,
            RdbcPool::Postgres(p) => p.get_conn().await,
        }
    }
    pub async fn find_list_by_query(&self, query: &RdbcQueryWrapper) -> OrmResp<Vec<RdbcOrmRow>> {
        match self {
            RdbcPool::Sqlite(p) => p.get_conn().await?.find_list_by_query(query).await,
            RdbcPool::Oracle(p) => p.get_conn().await?.find_list_by_query(query).await,
            RdbcPool::Mysql(p) => p.get_conn().await?.find_list_by_query(query).await,
            RdbcPool::Postgres(p) => p.get_conn().await?.find_list_by_query(query).await,
        }
    }
}

pub struct RdbcSqlitePool {
    pool: Pool<RusqliteConnectionManager>,
}
impl RdbcSqlitePool {
    async fn get_conn(&self) -> OrmResp<RdbcConnInner> {
        let conn_rs = self.pool.get().await;
        match conn_rs {
            Ok(conn) => {
                let conn = RdbcSqliteConn { conn };
                Ok(RdbcConnInner {
                    conn: RdbcConn::Sqlite(conn),
                })
            }
            Err(err) => Err(OrmError {
                kind: OrmErrorKind::PoolError,
                msg: err.to_string(),
            }),
        }
    }
}
pub struct RdbcMysqlPool {
    pool: Pool<MySqlConnectionManager>,
}
impl RdbcMysqlPool {
    async fn get_conn(&self) -> OrmResp<RdbcConnInner> {
        let conn_rs = self.pool.get().await;
        match conn_rs {
            Ok(conn) => {
                let mysql_conn = RdbcMysqlConn { conn };
                Ok(RdbcConnInner {
                    conn: RdbcConn::MySql(mysql_conn),
                })
            }
            Err(err) => Err(OrmError {
                kind: OrmErrorKind::PoolError,
                msg: err.to_string(),
            }),
        }
    }

    pub async fn find_list_by_query(&self, query: &RdbcQueryWrapper) -> OrmResp<Vec<RdbcOrmRow>> {
        self.get_conn().await?.find_list_by_query(query).await
    }
}
pub struct RdbcOraclePool {
    pool: Pool<OracleConnectionManager>,
}

impl RdbcOraclePool {
    async fn get_conn(&self) -> OrmResp<RdbcConnInner> {
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
pub struct RdbcPostgresPool {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}
impl RdbcPostgresPool {
    async fn get_conn(&self) -> OrmResp<RdbcConnInner> {
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

pub struct RdbcPoolInner {
    datasource: Arc<RdbcDataSource>,
    inner: RdbcPool,
}

impl RdbcPoolInner {
    pub async fn get_conn(&self) -> OrmResp<RdbcConnInner> {
        self.inner.get_conn().await
    }
    pub async fn find_list_by_query(&self, query: &RdbcQueryWrapper) -> OrmResp<Vec<RdbcOrmRow>> {
        self.get_conn().await?.find_list_by_query(query).await
    }
}

impl RdbcPoolInner {
    pub(crate) async fn new(datasource: Arc<RdbcDataSource>) -> OrmResp<RdbcPoolInner> {
        match datasource.db_type {
            RdbcDbType::Mysql => build_mysql_pool(datasource.clone()).await,
            RdbcDbType::Oracle => build_oracle_pool(datasource.clone()).await,
            RdbcDbType::Postgres => build_postgres_pool(datasource.clone()).await,
            RdbcDbType::Sqlite => build_sqlite_pool(datasource.clone()).await,
        }
    }
}

async fn build_sqlite_pool(data_source: Arc<RdbcDataSource>) -> OrmResp<RdbcPoolInner> {
    let manage = RusqliteConnectionManager::new("bmbp_msg.db");
    let pool_rs = Pool::builder()
        .max_size(data_source.pool_config.max_size.clone() as u32)
        .build(manage)
        .await;
    match pool_rs {
        Ok(pool) => Ok(RdbcPoolInner {
            datasource: data_source,
            inner: RdbcPool::Sqlite(RdbcSqlitePool { pool }),
        }),
        Err(err) => Err(OrmError {
            kind: OrmErrorKind::ConnError,
            msg: err.to_string(),
        }),
    }
}

async fn build_postgres_pool(data_source: Arc<RdbcDataSource>) -> OrmResp<RdbcPoolInner> {
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

async fn build_oracle_pool(data_source: Arc<RdbcDataSource>) -> OrmResp<RdbcPoolInner> {
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

async fn build_mysql_pool(data_source: Arc<RdbcDataSource>) -> OrmResp<RdbcPoolInner> {
    let opts = mysql_async::OptsBuilder::default()
        .ip_or_hostname(data_source.host.clone())
        .db_name(Some(data_source.db_name.clone()))
        .user(Some(data_source.user.clone()))
        .pass(Some(data_source.password.clone()))
        .tcp_port(data_source.port as u16);
    let manager = MySqlConnectionManager::new(Opts::from(opts));
    let pool_rs = Pool::builder()
        .max_size(data_source.pool_config.max_size.clone() as u32)
        .build(manager)
        .await;
    match pool_rs {
        Ok(pool) => Ok(RdbcPoolInner {
            datasource: data_source,
            inner: RdbcPool::Mysql(RdbcMysqlPool { pool }),
        }),
        Err(err) => Err(OrmError {
            kind: OrmErrorKind::ConnError,
            msg: err.to_string(),
        }),
    }
}

impl RdbcPoolInner {}
