use bb8::{ManageConnection, Pool};
use mysql_async::{prelude::Queryable, Conn, Opts, OptsBuilder};
use std::sync::Arc;

pub struct MySqlConnectionManager {
    opts: Opts,
}

impl MySqlConnectionManager {
    pub fn new(opts: Opts) -> Self {
        Self { opts: opts }
    }
}

#[async_trait::async_trait]
impl ManageConnection for MySqlConnectionManager {
    type Connection = Conn;
    type Error = mysql_async::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        Conn::new(self.opts.clone()).await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        conn.ping().await
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        false
    }
}
