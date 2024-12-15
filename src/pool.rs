use crate::bean::RdbcOrmRow;
use crate::ds::{RdbcDataSource, RdbcDbType};
use crate::error::{OrmError, OrmErrorKind, OrmResp};

use crate::client::{build_postgres_pool, RdbcPostgresPool};
use crate::{PageData, RdbcConn};
use bmbp_sql::RdbcQueryWrapper;
use std::sync::Arc;

pub enum RdbcPool {
    Postgres(RdbcPostgresPool),
}

impl RdbcPool {
    pub async fn new(datasource: Arc<RdbcDataSource>) -> OrmResp<RdbcPool> {
        match datasource.db_type {
            RdbcDbType::Postgres => build_postgres_pool(datasource.clone()).await,
            _ => {
                return Err(OrmError {
                    kind: OrmErrorKind::NotSupport,
                    msg: "不支持的数据库类型".to_string(),
                });
            }
        }
    }
    pub async fn get_conn(&self) -> OrmResp<RdbcConn> {
        match self {
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
