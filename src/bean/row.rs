use bmbp_sql::RdbcValue;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

/// 定义返回值类型
/// RdbcOrmRow 数据库查询结果 实现各个数据库的FromRow
/// RdbcPage 分页返回值

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct RdbcOrmRow {
    columns: Vec<String>,
    data: HashMap<String, RdbcValue>,
}
impl RdbcOrmRow {
    pub fn new() -> Self {
        RdbcOrmRow {
            columns: vec![],
            data: HashMap::new(),
        }
    }
    pub fn get_columns(&self) -> &Vec<String> {
        &self.columns
    }
    pub fn get_columns_mut(&mut self) -> &mut Vec<String> {
        &mut self.columns
    }
    pub fn get_data(&self) -> &HashMap<String, RdbcValue> {
        &self.data
    }
    pub fn get_data_mut(&mut self) -> &mut HashMap<String, RdbcValue> {
        &mut self.data
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PageData<T>
where
    T: Default + Debug + Clone + Serialize + From<RdbcOrmRow>,
{
    pub(crate) page_size: usize,
    pub(crate) page_num: usize,
    pub(crate) total: usize,
    pub(crate) data: Option<Vec<T>>,
}

impl<T> PageData<T> where T: Default + Debug + Clone + Serialize + From<RdbcOrmRow> {}

impl<T> PageData<T>
where
    T: Default + Debug + Clone + Serialize + From<RdbcOrmRow>,
{
    pub fn page_num(&self) -> &usize {
        &self.page_num
    }
    pub fn page_size(&self) -> &usize {
        &self.page_size
    }
    pub fn total(&self) -> &usize {
        &self.total
    }
    pub fn data(&self) -> &Option<Vec<T>> {
        &self.data
    }
    pub fn data_take(&mut self) -> Option<Vec<T>> {
        self.data.take()
    }
    pub fn set_page_num(&mut self, page_num: usize) -> &mut Self {
        self.page_num = page_num;
        self
    }
    pub fn set_page_size(&mut self, page_size: usize) -> &mut Self {
        self.page_size = page_size;
        self
    }
    pub fn set_total(&mut self, total: usize) -> &mut Self {
        self.total = total;
        self
    }
    pub fn set_data(&mut self, data: Option<Vec<T>>) -> &mut Self {
        self.data = data;
        self
    }
}
