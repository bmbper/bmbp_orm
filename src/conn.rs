use crate::bean::RdbcOrmRow;
use crate::client::{RdbcMysqlConn, RdbcOracleConn, RdbcPostgresConn, RdbcSqliteConn};
use crate::error::OrmResp;
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
}

pub trait RdbcTransConn {}

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
    pub(crate) async fn find_page_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
        page_num: usize,
        page_size: usize,
    ) -> OrmResp<PageData<RdbcOrmRow>> {
        self.conn
            .find_page_by_query(query, page_num, page_size)
            .await
    }
}

pub struct RdbcTransConnInner {
    pub conn: Box<dyn RdbcTransConn>,
}
