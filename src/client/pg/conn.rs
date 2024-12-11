use crate::error::{OrmError, OrmErrorKind, OrmResp};
use crate::{RdbcOrmRow, RdbcTransConn};
use bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use bmbp_sql::{render_query, DataBase, RdbcQueryWrapper};
use tokio_postgres::types::ToSql;
use tokio_postgres::NoTls;

pub struct RdbcPostgresConn<'a> {
    pub conn: PooledConnection<'a, PostgresConnectionManager<NoTls>>,
}
impl<'a> RdbcPostgresConn<'a> {
    pub(crate) async fn validate(&mut self) -> OrmResp<()> {
        Ok(())
    }
    pub(crate) async fn find_list_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
    ) -> OrmResp<Vec<RdbcOrmRow>> {
        let (sql, params) = render_query(query, DataBase::Postgres);
        let pg_prams = params
            .iter()
            .map(|v| v as &(dyn ToSql + Sync))
            .collect::<Vec<_>>();
        let rs = self.conn.query(sql.as_str(), &pg_prams).await;
        match rs {
            Ok(rows) => {
                let mut list = Vec::new();
                for row in rows {
                    let orm_row = RdbcOrmRow::from(row);
                    list.push(orm_row);
                }
                Ok(list)
            }
            Err(e) => Err(OrmError {
                kind: OrmErrorKind::SqlError,
                msg: e.to_string(),
            }),
        }
    }
}
pub struct RdbcPostgresTransConn {}
impl RdbcTransConn for RdbcPostgresTransConn {}
