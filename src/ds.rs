use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum RdbcDbType {
    Mysql,
    Oracle,
    Postgres,
    Sqlite,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RdbcDataSource {
    pub db_type: RdbcDbType,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub db_name: String,
    pub charset: String,
    pub pool_config: PoolConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PoolConfig {
    pub max_size: usize,
    pub min_size: usize,
    pub idle_timeout: Option<Duration>,
    pub max_lifetime: Option<Duration>,
}

impl Default for PoolConfig {
    fn default() -> Self {
        PoolConfig {
            max_size: 10,
            min_size: 1,
            idle_timeout: Some(Duration::from_secs(5)),
            max_lifetime: Some(Duration::from_secs(120)),
        }
    }
}
