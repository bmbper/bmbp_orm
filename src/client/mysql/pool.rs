use crate::adapter::MySqlConnectionManager;
use crate::client::mysql::RdbcMysqlConn;
use crate::error::{OrmError, OrmErrorKind, OrmResp};
use crate::{RdbcConn, RdbcDataSource, RdbcOrmRow, RdbcPool, RdbcTransaction};
use bb8::Pool;
use bmbp_sql::RdbcQueryWrapper;
use mysql_async::Opts;
use std::sync::Arc;

pub struct RdbcMysqlPool {
    datasource: Arc<RdbcDataSource>,
    pool: Pool<MySqlConnectionManager>,
}
impl RdbcMysqlPool {
    pub(crate) async fn get_conn(&self) -> OrmResp<RdbcConn> {
        let conn_rs = self.pool.get().await;
        match conn_rs {
            Ok(conn) => {
                let mysql_conn = RdbcMysqlConn { conn };
                Ok(RdbcConn::MySql(mysql_conn))
            }
            Err(err) => Err(OrmError {
                kind: OrmErrorKind::PoolError,
                msg: err.to_string(),
            }),
        }
    }

    pub async fn find_list_by_query(&self, query: &RdbcQueryWrapper) -> OrmResp<Vec<RdbcOrmRow>> {
        self.get_conn().await?.find_list_by_query(query).await
    }
}
pub async fn build_mysql_pool(data_source: Arc<RdbcDataSource>) -> OrmResp<RdbcPool> {
    let opts = mysql_async::OptsBuilder::default()
        .ip_or_hostname(data_source.host.clone())
        .db_name(Some(data_source.db_name.clone()))
        .user(Some(data_source.user.clone()))
        .pass(Some(data_source.password.clone()))
        .tcp_port(data_source.port as u16);
    let manager = MySqlConnectionManager::new(Opts::from(opts));
    let pool_rs = Pool::builder()
        .max_size(data_source.pool_config.max_size.clone() as u32)
        .build(manager)
        .await;
    match pool_rs {
        Ok(pool) => Ok(RdbcPool::Mysql(RdbcMysqlPool {
            pool,
            datasource: data_source.clone(),
        })),
        Err(err) => Err(OrmError {
            kind: OrmErrorKind::ConnError,
            msg: err.to_string(),
        }),
    }
}
