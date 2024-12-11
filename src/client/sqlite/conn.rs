use crate::error::OrmResp;
use crate::{RdbcOrmRow, RdbcTransConn};
use bb8::PooledConnection;
use bb8_sqlite::RusqliteConnectionManager;
use bmbp_sql::RdbcQueryWrapper;
pub struct RdbcSqliteConn<'a> {
    pub conn: PooledConnection<'a, RusqliteConnectionManager>,
}
impl<'a> RdbcSqliteConn<'a> {
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
pub struct RdbcSqliteTransConn {}
impl RdbcTransConn for RdbcSqliteTransConn {}
