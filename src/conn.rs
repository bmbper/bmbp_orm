use crate::adapter::MySqlConnectionManager;
use crate::error::{OrmError, OrmErrorKind, OrmResp};
use bb8::PooledConnection;
use bb8_oracle::OracleConnectionManager;
use bb8_postgres::PostgresConnectionManager;
use bb8_sqlite::RusqliteConnectionManager;
use bmbp_rdbc_type::RdbcOrmRow;
use bmbp_sql::{render_query, DataBase, RdbcQueryWrapper};
use tokio_postgres::types::ToSql;
use tokio_postgres::NoTls;

pub enum RdbcConn<'a> {
    MySql(RdbcMysqlConn<'a>),
    Oracle(RdbcOracleConn<'a>),
    Postgres(RdbcPostgresConn<'a>),
    Sqlite(RdbcSqliteConn<'a>),
}

impl<'a> RdbcConn<'a> {
    pub async fn validate(&mut self) -> OrmResp<()> {
        match self {
            RdbcConn::MySql(c) => c.validate().await,
            RdbcConn::Oracle(c) => c.validate().await,
            RdbcConn::Postgres(c) => c.validate().await,
            RdbcConn::Sqlite(c) => c.validate().await,
        }
    }
    pub(crate) async fn find_list_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
    ) -> OrmResp<Vec<RdbcOrmRow>> {
        match self {
            RdbcConn::MySql(c) => c.find_list_by_query(query).await,
            RdbcConn::Oracle(c) => c.find_list_by_query(query).await,
            RdbcConn::Postgres(c) => c.find_list_by_query(query).await,
            RdbcConn::Sqlite(c) => c.find_list_by_query(query).await,
        }
    }
}

pub struct RdbcSqliteConn<'a> {
    pub conn: PooledConnection<'a, RusqliteConnectionManager>,
}
impl<'a> RdbcSqliteConn<'a> {
    pub async fn validate(&mut self) -> OrmResp<()> {
        Ok(())
    }
    pub async fn find_list_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
    ) -> OrmResp<Vec<RdbcOrmRow>> {
        Ok(vec![])
    }
}
pub struct RdbcMysqlConn<'a> {
    pub conn: PooledConnection<'a, MySqlConnectionManager>,
}
impl<'a> RdbcMysqlConn<'a> {
    pub async fn validate(&mut self) -> OrmResp<()> {
        Ok(())
    }
    pub async fn find_list_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
    ) -> OrmResp<Vec<RdbcOrmRow>> {
        Ok(vec![])
    }
}
pub struct RdbcOracleConn<'a> {
    pub conn: PooledConnection<'a, OracleConnectionManager>,
}
impl<'a> RdbcOracleConn<'a> {
    pub async fn validate(&mut self) -> OrmResp<()> {
        Ok(())
    }
    pub async fn find_list_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
    ) -> OrmResp<Vec<RdbcOrmRow>> {
        Ok(vec![])
    }
}
pub struct RdbcPostgresConn<'a> {
    pub conn: PooledConnection<'a, PostgresConnectionManager<NoTls>>,
}
impl<'a> RdbcPostgresConn<'a> {
    async fn validate(&mut self) -> OrmResp<()> {
        Ok(())
    }
    async fn find_list_by_query(&mut self, query: &RdbcQueryWrapper) -> OrmResp<Vec<RdbcOrmRow>> {
        let (sql, params) = render_query(query, DataBase::Postgres);
        let pg_prams = params
            .iter()
            .map(|v| v as &(dyn ToSql + Sync))
            .collect::<Vec<_>>();
        let rs = self.conn.query(sql.as_str(), &pg_prams).await;
        match rs {
            Ok(rows) => {
                let mut list = Vec::new();
                for row in rows {
                    let orm_row = RdbcOrmRow::from(row);
                    list.push(orm_row);
                }
                Ok(list)
            }
            Err(e) => Err(OrmError {
                kind: OrmErrorKind::SqlError,
                msg: e.to_string(),
            }),
        }
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

pub struct RdbcConnInner<'a> {
    pub conn: RdbcConn<'a>,
}
impl<'a> RdbcConnInner<'a> {
    async fn validate(&mut self) -> OrmResp<()> {
        self.conn.validate().await
    }
    pub(crate) async fn find_list_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
    ) -> OrmResp<Vec<RdbcOrmRow>> {
        self.conn.find_list_by_query(query).await
    }
}

pub struct RdbcTransConnInner {
    pub conn: Box<dyn RdbcTransConn>,
}
