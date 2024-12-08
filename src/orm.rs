use crate::ds::RdbcDataSource;
use crate::error::OrmResp;
use crate::pool::RdbcPoolInner;
use crate::RdbcConnInner;
use bmbp_rdbc_type::RdbcValue;
use bmbp_sql::{RdbcDdlWrapper, RdbcDeleteWrapper, RdbcQueryWrapper};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub struct RdbcOrm {
    pool: RdbcPoolInner,
    datasource: Arc<RdbcDataSource>,
}

impl RdbcOrm {
    pub async fn new(datasource: Arc<RdbcDataSource>) -> OrmResp<Self> {
        let pool = RdbcPoolInner::new(datasource.clone()).await?;
        Ok(RdbcOrm {
            pool,
            datasource: datasource.clone(),
        })
    }
}

impl RdbcOrm {
    pub async fn get_conn(&self) -> OrmResp<RdbcConnInner> {
        self.pool.get_conn().await
    }
    pub async fn get_trans_conn(&self) {}
}
impl RdbcOrm {
    pub fn find_page_by_query(&self, query: &RdbcQueryWrapper, page_num: usize, page_size: usize) {}
    pub async fn find_list_by_query<T>(&self, query: &RdbcQueryWrapper) -> OrmResp<Vec<T>>
    where
        T: Serialize + for<'a> Deserialize<'a>,
    {
        self.pool.find_list_by_query(query).await
    }
    pub fn execute_delete_by_wrapper(&self, delete: &RdbcDeleteWrapper) {}
    pub fn execute_ddl_by_wrapper(&self, ddl: &RdbcDdlWrapper) {}
}

impl RdbcOrm {
    pub fn find_raw_page(
        &self,
        query: &String,
        params: Vec<RdbcValue>,
        page_num: usize,
        page_size: usize,
    ) {
    }
    pub fn find_raw_list(&self, query: &String, params: Vec<RdbcValue>) {}
    pub fn find_raw_one(&self, query: &String, params: Vec<RdbcValue>) {}
    pub fn execute_raw_insert(&self, insert: &String, params: Vec<RdbcValue>) {}
    pub fn execute_raw_update(&self, update: &String, params: Vec<RdbcValue>) {}
    pub fn execute_raw_delete(&self, delete: &String, params: Vec<RdbcValue>) {}
    pub fn execute_raw_ddl(&self, ddl: &String, params: Vec<RdbcValue>) {}
}

impl RdbcOrm {
    pub fn find_page_by_script(
        &self,
        query: &String,
        params: HashMap<String, RdbcValue>,
        page_num: usize,
        page_size: usize,
    ) {
    }
    pub fn find_list_by_script(&self, query: &String, params: HashMap<String, RdbcValue>) {}
    pub fn find_one_by_script(&self, query: &String, params: HashMap<String, RdbcValue>) {}
    pub fn execute_insert_script(&self, insert: &String, params: HashMap<String, RdbcValue>) {}
    pub fn execute_update_script(&self, update: &String, params: HashMap<String, RdbcValue>) {}
    pub fn execute_delete_script(&self, delete: &String, params: HashMap<String, RdbcValue>) {}
    pub fn execute_ddl_script(&self, ddl: &String, params: HashMap<String, RdbcValue>) {}
}
