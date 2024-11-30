use crate::error::{OrmError, OrmErrorKind, OrmResp};
use bmbp_sql::{render_query, DataBase, RdbcQueryWrapper};
use r2d2::PooledConnection;
use r2d2_mysql::MySqlConnectionManager;
use r2d2_oracle::OracleConnectionManager;
use r2d2_postgres::postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub enum RdbcConn {
    MySql(RdbcMysqlConn),
    Oracle(RdbcOracleConn),
    Postgres(RdbcPostgresConn),
    Sqlite(RdbcSqliteConn),
}

impl RdbcConn {
    pub(crate) fn validate(&mut self) -> OrmResp<()> {
        match self {
            RdbcConn::MySql(c) => c.validate(),
            RdbcConn::Oracle(c) => c.validate(),
            RdbcConn::Postgres(c) => c.validate(),
            RdbcConn::Sqlite(c) => c.validate(),
        }
    }
    pub(crate) fn find_list_by_query<T>(&mut self, query: &RdbcQueryWrapper) -> OrmResp<Vec<T>>
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        match self {
            RdbcConn::MySql(c) => c.find_list_by_query(query),
            RdbcConn::Oracle(c) => c.find_list_by_query(query),
            RdbcConn::Postgres(c) => c.find_list_by_query(query),
            RdbcConn::Sqlite(c) => c.find_list_by_query(query),
        }
    }
}

pub struct RdbcSqliteConn {
    pub conn: PooledConnection<SqliteConnectionManager>,
}
impl RdbcSqliteConn {
    pub fn validate(&mut self) -> OrmResp<()> {
        Ok(())
    }
    fn find_list_by_query<T>(&mut self, query: &RdbcQueryWrapper) -> OrmResp<Vec<T>>
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        Ok(vec![])
    }
}
pub struct RdbcMysqlConn {
    pub conn: PooledConnection<MySqlConnectionManager>,
}
impl RdbcMysqlConn {
    pub fn validate(&mut self) -> OrmResp<()> {
        Ok(())
    }
    fn find_list_by_query<T>(&mut self, query: &RdbcQueryWrapper) -> OrmResp<Vec<T>>
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        Ok(vec![])
    }
}
pub struct RdbcOracleConn {
    pub conn: PooledConnection<OracleConnectionManager>,
}
impl RdbcOracleConn {
    pub fn validate(&mut self) -> OrmResp<()> {
        Ok(())
    }
    fn find_list_by_query<T>(&mut self, query: &RdbcQueryWrapper) -> OrmResp<Vec<T>>
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        Ok(vec![])
    }
}
pub struct RdbcPostgresConn {
    pub conn: PooledConnection<PostgresConnectionManager<NoTls>>,
}
impl RdbcPostgresConn {
    fn validate(&mut self) -> OrmResp<()> {
        let _ = self.conn.is_valid(Duration::from_secs(100))?;
        Ok(())
    }
    fn find_list_by_query<T>(&mut self, query: &RdbcQueryWrapper) -> OrmResp<Vec<T>>
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        let (sql, params) = render_query(query, DataBase::Postgres);
        println!("===>sql:{}", sql);
        println!("====>params:{:#?}", params);
        Ok(vec![])
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
    pub conn: RdbcConn,
}
impl RdbcConnInner {
    pub fn validate(&mut self) -> OrmResp<()> {
        self.conn.validate()
    }
    pub(crate) fn find_list_by_query<T>(&mut self, query: &RdbcQueryWrapper) -> OrmResp<Vec<T>>
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        self.conn.find_list_by_query::<T>(query)
    }
}

pub struct RdbcTransConnInner {
    pub conn: Box<dyn RdbcTransConn>,
}
