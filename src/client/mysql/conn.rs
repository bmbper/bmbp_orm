use crate::adapter::MySqlConnectionManager;
use crate::error::OrmResp;
use crate::{PageData, RdbcOrmRow, RdbcTransConn};
use bb8::PooledConnection;
use bmbp_sql::RdbcQueryWrapper;

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
    pub(crate) async fn find_page_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
        page_num: usize,
        page_size: usize,
    ) -> OrmResp<PageData<RdbcOrmRow>> {
        Ok(PageData::default())
    }
}

pub struct RdbcMysqlTransConn {}
impl RdbcTransConn for RdbcMysqlTransConn {}
