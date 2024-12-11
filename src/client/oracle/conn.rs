use crate::error::OrmResp;
use crate::{RdbcOrmRow, RdbcTransConn};
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
}
pub struct RdbcOracleTransConn {}
impl RdbcTransConn for RdbcOracleTransConn {}
