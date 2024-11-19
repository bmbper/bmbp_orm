pub trait RdbcConn {}
pub struct RdbcSqliteConn {}
impl RdbcConn for RdbcSqliteConn {}
pub struct RdbcMysqlConn {}
impl RdbcConn for RdbcMysqlConn {}
pub struct RdbcOracleConn {}
impl RdbcConn for RdbcOracleConn {}
pub struct RdbcPostgresConn {}
impl RdbcConn for RdbcPostgresConn {}

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

pub struct RdbcTransConnInner {
    pub conn: Box<dyn RdbcTransConn>,
}
