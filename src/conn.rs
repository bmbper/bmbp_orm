use crate::adapter::MySqlConnectionManager;
use crate::error::{OrmError, OrmErrorKind, OrmResp};
use bb8::PooledConnection;
use bb8_oracle::OracleConnectionManager;
use bb8_postgres::PostgresConnectionManager;
use bb8_sqlite::RusqliteConnectionManager;
use bmbp_rdbc_type::RdbcValue;
use bmbp_sql::{render_query, DataBase, RdbcQueryWrapper};
use serde::{Deserialize, Serialize};
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
    pub(crate) async fn find_list_by_query<T>(
        &mut self,
        query: &RdbcQueryWrapper,
    ) -> OrmResp<Vec<T>>
    where
        T: Serialize + for<'t> Deserialize<'t>,
    {
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
    pub async fn find_list_by_query<T>(&mut self, query: &RdbcQueryWrapper) -> OrmResp<Vec<T>>
    where
        T: Serialize + for<'t> Deserialize<'t>,
    {
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
    async fn find_list_by_query<T>(&mut self, query: &RdbcQueryWrapper) -> OrmResp<Vec<T>>
    where
        T: Serialize + for<'t> Deserialize<'t>,
    {
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
    async fn find_list_by_query<T>(&mut self, query: &RdbcQueryWrapper) -> OrmResp<Vec<T>>
    where
        T: Serialize + for<'t> Deserialize<'t>,
    {
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
    async fn find_list_by_query<T>(&mut self, query: &RdbcQueryWrapper) -> OrmResp<Vec<T>>
    where
        T: Serialize + for<'t> Deserialize<'t>,
    {
        let (sql, params) = render_query(query, DataBase::Postgres);
        let pg_box_params = Self::convert_postgres_param(params);
        let params_ref: Vec<&(dyn ToSql + Sync)> = pg_box_params
            .iter()
            .map(|p| p.as_ref() as &(dyn ToSql + Sync))
            .collect();
        let rs = self.conn.query(sql.as_str(), params_ref.as_slice()).await;
        match rs {
            Ok(rows) => {
                for row in rows {
                    println!("==>{:#?}", row);
                }
                Ok(vec![])
            }
            Err(e) => {
                println!("==>{}", e.to_string());
                Err(OrmError {
                    kind: OrmErrorKind::SqlError,
                    msg: e.to_string(),
                })
            }
        }
    }
    pub fn convert_postgres_param(params: Vec<RdbcValue>) -> Vec<Box<(dyn ToSql + Sync + Send)>> {
        let mut params_v = vec![];
        for item in params {
            let p_v: Box<dyn ToSql + Sync + Send> = match item {
                RdbcValue::Char(c) => Box::new(c.to_string()),
                RdbcValue::Varchar(s) => Box::new(s),
                RdbcValue::Text(s) => Box::new(s),
                RdbcValue::LongText(s) => Box::new(s),
                RdbcValue::SmallInt(i) => Box::new(i),
                RdbcValue::Int(i) => Box::new(i),
                RdbcValue::BigInt(i) => Box::new(i),
                RdbcValue::Double(f) => Box::new(f.clone() as f64), // PostgreSQL uses f64 for real/float8
                RdbcValue::BigDouble(f) => Box::new(f),
                RdbcValue::Date(d) => Box::new(d),
                RdbcValue::DateTime(dt) => Box::new(dt),
                RdbcValue::Time(t) => Box::new(t),
                RdbcValue::TimeStamp(ts) => Box::new(ts.clone() as i64), // PostgreSQL uses bigint for timestamps
                RdbcValue::Bytes(b) => Box::new(b),
                RdbcValue::Boolean(b) => Box::new(b),
                RdbcValue::Array(arr) => {
                    let mut a_v = vec![];
                    for v in arr {
                        a_v.push(v.to_string());
                    }
                    Box::new(a_v)
                }
                RdbcValue::Object(obj) => match serde_json::to_string(&obj) {
                    Ok(v) => Box::new(v),
                    Err(e) => {
                        println!("==>{}", e.to_string());
                        Box::new("")
                    }
                },
                RdbcValue::Null => Box::new(Option::<i32>::None),
            };
            params_v.push(p_v);
        }

        params_v
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
impl RdbcConnInner<'_> {
    async fn validate(&mut self) -> OrmResp<()> {
        self.conn.validate().await
    }
    pub(crate) async fn find_list_by_query<T>(
        &mut self,
        query: &RdbcQueryWrapper,
    ) -> OrmResp<Vec<T>>
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        self.conn.find_list_by_query::<T>(query).await
    }
}

pub struct RdbcTransConnInner {
    pub conn: Box<dyn RdbcTransConn>,
}
