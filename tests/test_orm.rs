use bmbp_orm::{RdbcDataSource, RdbcDbType, RdbcOrm};
use std::panic;
use std::sync::Arc;

#[test]
pub fn test_orm() {
    let datasource = RdbcDataSource {
        db_type: RdbcDbType::Mysql,
        host: "127.0.0.1".to_string(),
        port: 5432,
        user: "bmbp".to_string(),
        password: "zgk0130!".to_string(),
        db_name: "bmbp".to_string(),
        charset: "".to_string(),
        pool_config: Default::default(),
    };

    let orm_rs = RdbcOrm::new(Arc::new(datasource));
    match orm_rs {
        Ok(orm) => {
            let conn_rs = orm.get_conn();
            match conn_rs {
                Ok(mut conn) => match conn.validate() {
                    Ok(_) => println!("连接成功"),
                    Err(e) => {
                        panic!("{:#?}", e)
                    }
                },
                Err(err) => println!("{:?}", err),
            }
        }
        Err(err) => println!("{:?}", err),
    }
}
