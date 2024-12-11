use crate::adapter::MySqlConnectionManager;
use crate::error::OrmResp;
use crate::{RdbcOrmRow, RdbcTransConn};
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
}

pub struct RdbcMysqlTransConn {}
impl RdbcTransConn for RdbcMysqlTransConn {}
