use crate::error::{OrmError, OrmErrorKind, OrmResp};
use crate::{PageData, RdbcOrmRow, RdbcTransaction};
use bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use bmbp_sql::{
    render_delete, render_insert, render_query, render_update, DataBase, RdbcDeleteWrapper,
    RdbcInsertWrapper, RdbcQueryWrapper, RdbcUpdateWrapper, RdbcValue,
};
use tokio_postgres::types::ToSql;
use tokio_postgres::{NoTls, Transaction};

pub struct RdbcPostgresConn<'a> {
    pub conn: PooledConnection<'a, PostgresConnectionManager<NoTls>>,
}
impl<'a> RdbcPostgresConn<'a> {
    pub(crate) async fn validate(&mut self) -> OrmResp<()> {
        Ok(())
    }
    pub async fn get_transaction(&mut self) -> OrmResp<RdbcTransaction> {
        let trans = self.conn.transaction().await?;
        let conn = RdbcPostgresTransaction { trans: Some(trans) };
        Ok(RdbcTransaction::Postgres(conn))
    }
    pub(crate) async fn find_page_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
        page_num: usize,
        page_size: usize,
    ) -> OrmResp<PageData<RdbcOrmRow>> {
        let (sql, params) = render_query(query, DataBase::Postgres);
        let pg_prams = params
            .iter()
            .map(|v| v as &(dyn ToSql + Sync))
            .collect::<Vec<_>>();
        let mut page_data = PageData::<RdbcOrmRow> {
            page_num: page_num.clone(),
            page_size: page_size.clone(),
            total: 0,
            data: None,
        };

        match self.find_count_by_sql_pg_params(&sql, &pg_prams).await {
            Ok(count) => {
                page_data.total = count;
            }
            Err(e) => {
                return Err(OrmError {
                    kind: OrmErrorKind::SqlError,
                    msg: e.to_string(),
                });
            }
        }
        let new_page_num = {
            if page_num > 1 {
                page_num
            } else {
                1
            }
        };
        let new_page_size = {
            if page_size > 0 {
                page_size
            } else {
                10
            }
        };
        let page_sql = format!(
            "SELECT * FROM ({}) LIMIT {} OFFSET {}",
            sql,
            new_page_size.clone(),
            (new_page_num - 1) * new_page_size
        );
        let row_vec = self
            .find_list_by_raw_sql_pg_params(&page_sql, &pg_prams)
            .await?;
        page_data.data = Some(row_vec);
        Ok(page_data)
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
        self.find_list_by_raw_sql_pg_params(&sql, &pg_prams).await
    }
    pub(crate) async fn find_one_by_query(
        &mut self,
        query: &RdbcQueryWrapper,
    ) -> OrmResp<Option<RdbcOrmRow>> {
        let (sql, params) = render_query(query, DataBase::Postgres);
        let pg_prams = params
            .iter()
            .map(|v| v as &(dyn ToSql + Sync))
            .collect::<Vec<_>>();
        self.find_one_by_raw_sql_pg_params(&sql, &pg_prams).await
    }

    pub(crate) async fn find_count_by_query(&mut self, query: &RdbcQueryWrapper) -> OrmResp<usize> {
        let (sql, params) = render_query(query, DataBase::Postgres);
        let pg_prams = params
            .iter()
            .map(|v| v as &(dyn ToSql + Sync))
            .collect::<Vec<_>>();
        self.find_count_by_sql_pg_params(&sql, &pg_prams).await
    }

    pub(crate) async fn find_list_by_raw_sql_pg_params(
        &mut self,
        sql: &String,
        params: &Vec<&(dyn ToSql + Sync)>,
    ) -> OrmResp<Vec<RdbcOrmRow>> {
        let rs = self.conn.query(sql.as_str(), &params).await;
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
    pub(crate) async fn find_one_by_raw_sql_pg_params(
        &mut self,
        sql: &str,
        params: &Vec<&(dyn ToSql + Sync)>,
    ) -> OrmResp<Option<RdbcOrmRow>> {
        let count_sql = format!("SELECT COUNT(1) FROM ({}) AS count ", sql);
        match self.conn.query_opt(count_sql.as_str(), params).await {
            Ok(row_op) => {
                if let Some(row) = row_op {
                    Ok(Some(RdbcOrmRow::from(row)))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(OrmError {
                kind: OrmErrorKind::SqlError,
                msg: e.to_string(),
            }),
        }
    }
    pub(crate) async fn find_count_by_sql_pg_params(
        &mut self,
        sql: &String,
        params: &Vec<&(dyn ToSql + Sync)>,
    ) -> OrmResp<usize> {
        if let Some(total_row) = self.find_one_by_raw_sql_pg_params(&sql, &params).await? {
            if let Some(total_value) = total_row.get_data().get("count") {
                if let Some(total_value) = total_value.as_number() {
                    Ok(total_value as usize)
                } else {
                    return Err(OrmError {
                        kind: OrmErrorKind::SqlError,
                        msg: "查询总数失败: 记录数值解析异常".to_string(),
                    });
                }
            } else {
                return Err(OrmError {
                    kind: OrmErrorKind::SqlError,
                    msg: "查询总数失败: 未获取到记录数".to_string(),
                });
            }
        } else {
            return Err(OrmError {
                kind: OrmErrorKind::SqlError,
                msg: "查询总数失败: 未查询到统计记录".to_string(),
            });
        }
    }
    pub(crate) async fn execute_insert_by_wrapper(
        &mut self,
        insert: &RdbcInsertWrapper,
    ) -> OrmResp<usize> {
        let (sql, params) = render_insert(insert, DataBase::Postgres);
        self.execute_sql_params(&sql, &params).await
    }
    pub(crate) async fn execute_update_by_wrapper(
        &mut self,
        update: &RdbcUpdateWrapper,
    ) -> OrmResp<usize> {
        let (sql, params) = render_update(update, DataBase::Postgres);
        self.execute_sql_params(&sql, &params).await
    }
    pub(crate) async fn execute_delete_by_wrapper(
        &mut self,
        delete: &RdbcDeleteWrapper,
    ) -> OrmResp<usize> {
        let (sql, params) = render_delete(delete, DataBase::Postgres);
        self.execute_sql_params(&sql, &params).await
    }

    pub(crate) async fn execute_sql_params(
        &mut self,
        sql: &String,
        params: &Vec<RdbcValue>,
    ) -> OrmResp<usize> {
        let pg_prams = params
            .iter()
            .map(|v| v as &(dyn ToSql + Sync))
            .collect::<Vec<_>>();
        let row_count = self.conn.execute(sql, &pg_prams).await?;
        Ok(row_count as usize)
    }
}

pub struct RdbcPostgresTransaction<'a> {
    pub trans: Option<Transaction<'a>>,
}
impl<'a> RdbcPostgresTransaction<'a> {
    pub async fn commit(&mut self) -> OrmResp<()> {
        if let Some(trans) = self.trans.take() {
            trans.commit().await?;
            Ok(())
        } else {
            Err(OrmError {
                kind: OrmErrorKind::SqlError,
                msg: "Transaction already completed.".to_string(),
            })
        }
    }

    pub async fn rollback(&mut self) -> OrmResp<()> {
        if let Some(trans) = self.trans.take() {
            trans.rollback().await?;
            Ok(())
        } else {
            Err(OrmError {
                kind: OrmErrorKind::SqlError,
                msg: "Transaction already completed.".to_string(),
            })
        }
    }
}
