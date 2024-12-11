use crate::bean::RdbcOrmRow;
use crate::ds::{RdbcDataSource, RdbcDbType};
use crate::error::OrmResp;

use crate::client::{
    build_mysql_pool, build_oracle_pool, build_postgres_pool, build_sqlite_pool, RdbcMysqlPool,
    RdbcOraclePool, RdbcPostgresPool, RdbcSqlitePool,
};
use crate::RdbcConnInner;
use bmbp_sql::RdbcQueryWrapper;
use std::sync::Arc;

pub enum RdbcPool {
    Sqlite(RdbcSqlitePool),
    Oracle(RdbcOraclePool),
    Mysql(RdbcMysqlPool),
    Postgres(RdbcPostgresPool),
}

impl RdbcPool {
    pub async fn get_conn(&self) -> OrmResp<RdbcConnInner> {
        match self {
            RdbcPool::Sqlite(p) => p.get_conn().await,
            RdbcPool::Oracle(p) => p.get_conn().await,
            RdbcPool::Mysql(p) => p.get_conn().await,
            RdbcPool::Postgres(p) => p.get_conn().await,
        }
    }
    pub async fn find_list_by_query(&self, query: &RdbcQueryWrapper) -> OrmResp<Vec<RdbcOrmRow>> {
        match self {
            RdbcPool::Sqlite(p) => p.get_conn().await?.find_list_by_query(query).await,
            RdbcPool::Oracle(p) => p.get_conn().await?.find_list_by_query(query).await,
            RdbcPool::Mysql(p) => p.get_conn().await?.find_list_by_query(query).await,
            RdbcPool::Postgres(p) => p.get_conn().await?.find_list_by_query(query).await,
        }
    }
}

pub struct RdbcPoolInner {
    pub(crate) datasource: Arc<RdbcDataSource>,
    pub(crate) inner: RdbcPool,
}

impl RdbcPoolInner {
    pub async fn get_conn(&self) -> OrmResp<RdbcConnInner> {
        self.inner.get_conn().await
    }
    pub async fn find_list_by_query(&self, query: &RdbcQueryWrapper) -> OrmResp<Vec<RdbcOrmRow>> {
        self.get_conn().await?.find_list_by_query(query).await
    }
}

impl RdbcPoolInner {
    pub(crate) async fn new(datasource: Arc<RdbcDataSource>) -> OrmResp<RdbcPoolInner> {
        match datasource.db_type {
            RdbcDbType::Mysql => build_mysql_pool(datasource.clone()).await,
            RdbcDbType::Oracle => build_oracle_pool(datasource.clone()).await,
            RdbcDbType::Postgres => build_postgres_pool(datasource.clone()).await,
            RdbcDbType::Sqlite => build_sqlite_pool(datasource.clone()).await,
        }
    }
}

impl RdbcPoolInner {}
