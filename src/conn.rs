use crate::bean::RdbcOrmRow;
use crate::client::{
    RdbcMysqlConn, RdbcMysqlTransaction, RdbcOracleConn, RdbcOracleTransaction, RdbcPostgresConn,
    RdbcPostgresTransaction, RdbcSqliteConn, RdbcSqliteTransaction,
};
use crate::error::{OrmError, OrmErrorKind, OrmResp};
use crate::PageData;
use bmbp_sql::RdbcQueryWrapper;

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
    pub async fn get_transaction(&mut self) -> OrmResp<RdbcTransaction> {
        match self {
            RdbcConn::MySql(c) => Err(OrmError {
                kind: OrmErrorKind::NotImplement,
                msg: "mysql not implement".to_string(),
            }),
            RdbcConn::Oracle(c) => Err(OrmError {
                kind: OrmErrorKind::NotImplement,
                msg: "mysql not implement".to_string(),
            }),
            RdbcConn::Postgres(c) => c.get_transaction().await,
            RdbcConn::Sqlite(c) => Err(OrmError {
                kind: OrmErrorKind::NotImplement,
                msg: "mysql not implement".to_string(),
            }),
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
    pub(crate) async fn find_page_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
        page_num: usize,
        page_size: usize,
    ) -> OrmResp<PageData<RdbcOrmRow>> {
        match self {
            RdbcConn::MySql(c) => c.find_page_by_query(query, page_num, page_size).await,
            RdbcConn::Oracle(c) => c.find_page_by_query(query, page_num, page_size).await,
            RdbcConn::Postgres(c) => c.find_page_by_query(query, page_num, page_size).await,
            RdbcConn::Sqlite(c) => c.find_page_by_query(query, page_num, page_size).await,
        }
    }
    pub(crate) async fn find_one_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
    ) -> OrmResp<Option<RdbcOrmRow>> {
        match self {
            RdbcConn::MySql(c) => c.find_one_by_query(query).await,
            RdbcConn::Oracle(c) => c.find_one_by_query(query).await,
            RdbcConn::Postgres(c) => c.find_one_by_query(query).await,
            RdbcConn::Sqlite(c) => c.find_one_by_query(query).await,
        }
    }
}
pub enum RdbcTransaction<'a> {
    MySql(RdbcMysqlTransaction<'a>),
    Oracle(RdbcOracleTransaction<'a>),
    Postgres(RdbcPostgresTransaction<'a>),
    Sqlite(RdbcSqliteTransaction<'a>),
}
