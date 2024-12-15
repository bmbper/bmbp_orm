use crate::bean::RdbcOrmRow;
use crate::ds::{RdbcDataSource, RdbcDbType};
use crate::error::OrmResp;

use crate::client::{
    build_mysql_pool, build_oracle_pool, build_postgres_pool, build_sqlite_pool, RdbcMysqlPool,
    RdbcOraclePool, RdbcPostgresPool, RdbcSqlitePool,
};
use crate::{PageData, RdbcConn, RdbcTransaction};
use bmbp_sql::RdbcQueryWrapper;
use std::sync::Arc;

pub enum RdbcPool {
    Sqlite(RdbcSqlitePool),
    Oracle(RdbcOraclePool),
    Mysql(RdbcMysqlPool),
    Postgres(RdbcPostgresPool),
}

impl RdbcPool {
    pub async fn new(datasource: Arc<RdbcDataSource>) -> OrmResp<RdbcPool> {
        match datasource.db_type {
            RdbcDbType::Mysql => build_mysql_pool(datasource.clone()).await,
            RdbcDbType::Oracle => build_oracle_pool(datasource.clone()).await,
            RdbcDbType::Postgres => build_postgres_pool(datasource.clone()).await,
            RdbcDbType::Sqlite => build_sqlite_pool(datasource.clone()).await,
        }
    }
    pub async fn get_conn(&self) -> OrmResp<RdbcConn> {
        match self {
            RdbcPool::Sqlite(p) => p.get_conn().await,
            RdbcPool::Oracle(p) => p.get_conn().await,
            RdbcPool::Mysql(p) => p.get_conn().await,
            RdbcPool::Postgres(p) => p.get_conn().await,
        }
    }

    pub async fn find_list_by_query(&self, query: &RdbcQueryWrapper) -> OrmResp<Vec<RdbcOrmRow>> {
        self.get_conn().await?.find_list_by_query(query).await
    }
    pub async fn find_page_by_query(
        &self,
        query: &RdbcQueryWrapper,
        page_num: usize,
        page_size: usize,
    ) -> OrmResp<PageData<RdbcOrmRow>> {
        self.get_conn()
            .await?
            .find_page_by_query(query, page_num, page_size)
            .await
    }
    pub(crate) async fn find_one_by_query(
        &self,
        query: &RdbcQueryWrapper,
    ) -> OrmResp<Option<RdbcOrmRow>> {
        self.get_conn().await?.find_one_by_query(query).await
    }
}
