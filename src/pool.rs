use crate::ds::{RdbcDataSource, RdbcDbType};
use crate::error::OrmResp;
use crate::{RdbcConnInner, RdbcMysqlConn, RdbcOracleConn, RdbcPostgresConn, RdbcSqliteConn};
use bmbp_bean::BmbpResp;
use bmbp_sql::RdbcQueryWrapper;
use r2d2::Pool;
use r2d2_mysql::{mysql, MySqlConnectionManager};
use r2d2_oracle::OracleConnectionManager;
use r2d2_postgres::postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use r2d2_redis::redis::Commands;
use r2d2_sqlite::SqliteConnectionManager;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub trait RdbcPool: Sync + Send {
    fn get_conn(&self) -> OrmResp<RdbcConnInner>;
    fn find_list_by_query<T>(&self, query: &RdbcQueryWrapper) -> BmbpResp<Vec<T>>
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        self.get_conn().find_list_by_query(query)
    }
}
pub struct RdbcSqlitePool {
    pool: Pool<SqliteConnectionManager>,
}
impl RdbcPool for RdbcSqlitePool {
    fn get_conn(&self) -> OrmResp<RdbcConnInner> {
        let conn = RdbcSqliteConn {
            conn: self.pool.get()?,
        };
        Ok(RdbcConnInner {
            conn: Box::new(conn),
        })
    }
}
pub struct RdbcMysqlPool {
    pool: Pool<MySqlConnectionManager>,
}
impl RdbcPool for RdbcMysqlPool {
    fn get_conn(&self) -> OrmResp<RdbcConnInner> {
        let conn = RdbcMysqlConn {
            conn: self.pool.get()?,
        };
        Ok(RdbcConnInner {
            conn: Box::new(conn),
        })
    }
}
pub struct RdbcOraclePool {
    pool: Pool<OracleConnectionManager>,
}
impl RdbcPool for RdbcOraclePool {
    fn get_conn(&self) -> OrmResp<RdbcConnInner> {
        let conn = RdbcOracleConn {
            conn: self.pool.get()?,
        };
        Ok(RdbcConnInner {
            conn: Box::new(conn),
        })
    }
}
pub struct RdbcPostgresPool {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}
impl RdbcPool for RdbcPostgresPool {
    fn get_conn(&self) -> OrmResp<RdbcConnInner> {
        let conn = RdbcPostgresConn {
            conn: self.pool.get()?,
        };
        Ok(RdbcConnInner {
            conn: Box::new(conn),
        })
    }
}

pub struct RdbcPoolInner {
    datasource: Arc<RdbcDataSource>,
    inner: Box<dyn RdbcPool>,
}

impl RdbcPoolInner {
    pub fn get_conn(&self) -> OrmResp<RdbcConnInner> {
        self.inner.get_conn()
    }
    pub fn find_list_by_query<T>(&self, query: &RdbcQueryWrapper) -> BmbpResp<Vec<T>>
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        self.inner.find_list_by_query(query)
    }
}

impl RdbcPoolInner {
    pub(crate) fn new(datasource: Arc<RdbcDataSource>) -> OrmResp<RdbcPoolInner> {
        match datasource.db_type {
            RdbcDbType::Mysql => build_mysql_pool(datasource.clone()),
            RdbcDbType::Oracle => build_oracle_pool(datasource.clone()),
            RdbcDbType::Postgres => build_postgres_pool(datasource.clone()),
            RdbcDbType::Sqlite => build_sqlite_pool(datasource.clone()),
        }
    }
}

fn build_sqlite_pool(data_source: Arc<RdbcDataSource>) -> OrmResp<RdbcPoolInner> {
    let manager = SqliteConnectionManager::file("bmbp_msg.db");
    let poo_rs = Pool::new(manager)?;
    Ok(RdbcPoolInner {
        datasource: data_source,
        inner: Box::new(RdbcSqlitePool { pool: poo_rs }),
    })
}

fn build_postgres_pool(data_source: Arc<RdbcDataSource>) -> OrmResp<RdbcPoolInner> {
    let conn_str = format!(
        "host={} port={} user={} password={} dbname={}",
        data_source.host,
        data_source.port,
        data_source.user,
        data_source.password,
        data_source.db_name
    );
    let manage = PostgresConnectionManager::new(conn_str.as_str().parse().unwrap(), NoTls);
    let pool = Pool::new(manage)?;
    Ok(RdbcPoolInner {
        datasource: data_source,
        inner: Box::new(RdbcPostgresPool { pool }),
    })
}

fn build_oracle_pool(data_source: Arc<RdbcDataSource>) -> OrmResp<RdbcPoolInner> {
    let manager = OracleConnectionManager::new(
        data_source.user.as_str(),
        data_source.password.as_str(),
        data_source.host.as_str(),
    );
    let pool = Pool::new(manager)?;
    Ok(RdbcPoolInner {
        datasource: data_source,
        inner: Box::new(RdbcOraclePool { pool }),
    })
}

fn build_mysql_pool(data_source: Arc<RdbcDataSource>) -> OrmResp<RdbcPoolInner> {
    let opts = mysql::OptsBuilder::new()
        .ip_or_hostname(Some(data_source.host.clone()))
        .db_name(Some(data_source.db_name.clone()))
        .user(Some(data_source.user.clone()))
        .pass(Some(data_source.password.clone()))
        .tcp_port(data_source.port as u16);
    let manager = r2d2_mysql::MySqlConnectionManager::new(opts);
    let pool = Pool::new(manager)?;
    Ok(RdbcPoolInner {
        datasource: data_source,
        inner: Box::new(RdbcMysqlPool { pool }),
    })
}

impl RdbcPoolInner {}
