use crate::bean::RdbcOrmRow;
use crate::client::{RdbcPostgresConn, RdbcPostgresTransaction};
use crate::error::OrmResp;
use crate::PageData;
use bmbp_sql::{RdbcDeleteWrapper, RdbcInsertWrapper, RdbcQueryWrapper, RdbcUpdateWrapper};

pub enum RdbcConn<'a> {
    Postgres(RdbcPostgresConn<'a>),
}

impl<'a> RdbcConn<'a> {
    pub async fn validate(&mut self) -> OrmResp<()> {
        match self {
            RdbcConn::Postgres(c) => c.validate().await,
        }
    }
    pub async fn get_transaction(&mut self) -> OrmResp<RdbcTransaction> {
        match self {
            RdbcConn::Postgres(c) => c.get_transaction().await,
        }
    }
    pub(crate) async fn find_list_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
    ) -> OrmResp<Vec<RdbcOrmRow>> {
        match self {
            RdbcConn::Postgres(c) => c.find_list_by_query(query).await,
        }
    }
    pub(crate) async fn find_page_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
        page_num: usize,
        page_size: usize,
    ) -> OrmResp<PageData<RdbcOrmRow>> {
        match self {
            RdbcConn::Postgres(c) => c.find_page_by_query(query, page_num, page_size).await,
        }
    }
    pub(crate) async fn find_one_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
    ) -> OrmResp<Option<RdbcOrmRow>> {
        match self {
            RdbcConn::Postgres(c) => c.find_one_by_query(query).await,
        }
    }
    pub(crate) async fn execute_insert_by_wrapper(
        &mut self,
        insert: &RdbcInsertWrapper,
    ) -> OrmResp<usize> {
        match self {
            RdbcConn::Postgres(c) => c.execute_insert_by_wrapper(insert).await,
        }
    }
    pub(crate) async fn execute_update_by_wrapper(
        &mut self,
        update: &RdbcUpdateWrapper,
    ) -> OrmResp<usize> {
        match self {
            RdbcConn::Postgres(c) => c.execute_update_by_wrapper(update).await,
        }
    }
    pub(crate) async fn execute_delete_by_wrapper(
        &mut self,
        delete: &RdbcDeleteWrapper,
    ) -> OrmResp<usize> {
        match self {
            RdbcConn::Postgres(c) => c.execute_delete_by_wrapper(delete).await,
        }
    }
}
pub enum RdbcTransaction<'a> {
    Postgres(RdbcPostgresTransaction<'a>),
}

impl<'a> RdbcTransaction<'a> {
    pub async fn commit(&mut self) -> OrmResp<()> {
        match self {
            RdbcTransaction::Postgres(c) => c.commit().await,
        }
    }
}
