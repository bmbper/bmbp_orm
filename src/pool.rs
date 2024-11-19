use crate::ds::{RdbcDataSource, RdbcDbType};
use r2d2::Pool;
use r2d2_mysql::MySqlConnectionManager;
use r2d2_oracle::OracleConnectionManager;
use r2d2_postgres::postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_sqlite::SqliteConnectionManager;
use std::sync::Arc;

pub trait RdbcPool {}
pub struct RdbcSqlitePool {
    pool: Pool<SqliteConnectionManager>,
}
impl RdbcPool for RdbcSqlitePool {}
pub struct RdbcMysqlPool {
    pool: Pool<MySqlConnectionManager>,
}
impl RdbcPool for RdbcMysqlPool {}
pub struct RdbcOraclePool {
    pool: Pool<OracleConnectionManager>,
}
impl RdbcPool for RdbcOraclePool {}
pub struct RdbcPostgresPool {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}
impl RdbcPool for RdbcPostgresPool {}

pub struct RdbcPoolInner {
    datasource: Arc<RdbcDataSource>,
    inner: Box<dyn RdbcPool>,
}

impl RdbcPoolInner {
    pub(crate) fn new(datasource: Arc<RdbcDataSource>) -> RdbcPoolInner {
        match datasource.db_type {
            RdbcDbType::Mysql => build_mysql_pool(datasource.clone()),
            RdbcDbType::Oracle => build_oracle_pool(datasource.clone()),
            RdbcDbType::Postgres => build_postgres_pool(datasource.clone()),
            RdbcDbType::Sqlite => build_sqlite_pool(datasource.clone()),
        }
    }
}

fn build_sqlite_pool(data_source: Arc<RdbcDataSource>) -> BmbpResp<RdbcPoolInner> {
    let manager = SqliteConnectionManager::file("bmbp_msg.db");
    let poo_rs = Pool::new(manager)?;
    RdbcPoolInner {
        datasource: data_source,
        inner: Box::new(RdbcSqlitePool { pool: poo_rs }),
    }
}

fn build_postgres_pool(data_source: Arc<RdbcDataSource>) -> RdbcPoolInner {
    todo!()
}

fn build_oracle_pool(data_source: Arc<RdbcDataSource>) -> RdbcPoolInner {
    todo!()
}

fn build_mysql_pool(data_source: Arc<RdbcDataSource>) -> RdbcPoolInner {
    todo!()
}

impl RdbcPoolInner {}
