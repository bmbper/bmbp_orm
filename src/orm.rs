use crate::ds::RdbcDataSource;
use crate::pool::{RdbcPool, RdbcPoolInner};
use bmbp_sql::{
    RdbcDdlWrapper, RdbcDeleteWrapper, RdbcInsertWrapper, RdbcQueryWrapper, RdbcUpdateWrapper,
    RdbcValue,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct RdbcOrm {
    pool: RdbcPoolInner,
    datasource: Arc<RdbcDataSource>,
}

impl RdbcOrm {
    pub fn new(datasource: Arc<RdbcDataSource>) -> Self {
        RdbcOrm {
            pool: RdbcPoolInner::new(datasource.clone()),
            datasource: datasource.clone(),
        }
    }
}

impl RdbcOrm {
    pub fn get_conn(&self) {}
    pub fn get_trans_conn(&self) {}
}
impl RdbcOrm {
    pub fn find_page_by_query(&self, query: &RdbcQueryWrapper, page_num: usize, page_size: usize) {}
    pub fn find_list_by_query(&self, query: &RdbcQueryWrapper) {}
    pub fn find_one_by_query(&self, query: &RdbcQueryWrapper) {}
    pub fn execute_insert_by_wrapper(&self, insert: &RdbcInsertWrapper) {}
    pub fn execute_update_by_wrapper(&self, update: &RdbcUpdateWrapper) {}
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
