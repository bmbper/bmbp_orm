use crate::error::OrmResp;
use crate::{PageData, RdbcOrmRow};
use bb8::PooledConnection;
use bb8_oracle::OracleConnectionManager;
use bmbp_sql::RdbcQueryWrapper;

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
    pub(crate) async fn find_page_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
        page_num: usize,
        page_size: usize,
    ) -> OrmResp<PageData<RdbcOrmRow>> {
        Ok(PageData::default())
    }
    pub(crate) async fn find_one_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
    ) -> OrmResp<Option<RdbcOrmRow>> {
        Ok(None)
    }
}
pub struct RdbcOracleTransaction<'a> {
    transaction: RdbcOracleConn<'a>,
}
