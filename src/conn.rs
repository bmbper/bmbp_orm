use crate::error::{OrmError, OrmErrorKind, OrmResp};
use r2d2::PooledConnection;
use r2d2_mysql::MySqlConnectionManager;
use r2d2_oracle::OracleConnectionManager;
use r2d2_postgres::postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_sqlite::SqliteConnectionManager;
use std::time::Duration;

pub trait RdbcConn {
    fn validate(&mut self) -> OrmResp<()> {
        Err(OrmError {
            kind: OrmErrorKind::ConnError,
            msg: "".to_string(),
        })
    }
}
pub struct RdbcSqliteConn {
    pub conn: PooledConnection<SqliteConnectionManager>,
}
impl RdbcConn for RdbcSqliteConn {}
pub struct RdbcMysqlConn {
    pub conn: PooledConnection<MySqlConnectionManager>,
}
impl RdbcConn for RdbcMysqlConn {}
pub struct RdbcOracleConn {
    pub conn: PooledConnection<OracleConnectionManager>,
}
impl RdbcConn for RdbcOracleConn {}
pub struct RdbcPostgresConn {
    pub conn: PooledConnection<PostgresConnectionManager<NoTls>>,
}
impl RdbcConn for RdbcPostgresConn {
    fn validate(&mut self) -> OrmResp<()> {
        let _ = self.conn.is_valid(Duration::from_secs(100))?;
        Ok(())
    }
}

pub trait RdbcTransConn {}
pub struct RdbcSqliteTransConn {}
impl RdbcTransConn for RdbcSqliteTransConn {}
pub struct RdbcPostgresTransConn {}
impl RdbcTransConn for RdbcPostgresTransConn {}
pub struct RdbcMysqlTransConn {}
impl RdbcTransConn for RdbcMysqlTransConn {}
pub struct RdbcOracleTransConn {}
impl RdbcTransConn for RdbcOracleTransConn {}

pub struct RdbcConnInner {
    pub conn: Box<dyn RdbcConn>,
}
impl RdbcConnInner {
    pub fn validate(&mut self) -> OrmResp<()> {
        self.conn.validate()
    }
}

pub struct RdbcTransConnInner {
    pub conn: Box<dyn RdbcTransConn>,
}
