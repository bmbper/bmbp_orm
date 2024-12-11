use crate::bean::row::RdbcOrmRow;
use bmbp_sql::RdbcValue;
use tokio_postgres::Row;

impl From<Row> for RdbcOrmRow {
    fn from(row: Row) -> Self {
        let mut orm_row = RdbcOrmRow::new();
        let columns = row.columns();
        for col in columns {
            let col_name = col.name().to_string();
            orm_row.get_columns_mut().push(col_name.clone());
            let col_type = col.type_().name().to_string();
            match col_type.as_str() {
                "text" | "varchar" | "char" | "json" | "xml" => {
                    let col_value: Option<String> = row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::Varchar(value));
                    }
                }
                "int2" => {
                    let col_value: Option<i16> = row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::Int(value as i32));
                    }
                }
                "int4" => {
                    let col_value: Option<i32> = row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::BigInt(value as i64));
                    }
                }
                "int8" => {
                    let col_value: Option<i64> = row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::BigInt(value));
                    }
                }
                "float4" | "float8" => {
                    let col_value: Option<f32> = row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::Double(value));
                    }
                }
                "date" | "time" | "timestamp" => {
                    let col_value: Option<chrono::DateTime<chrono::Utc>> =
                        row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::DateTime(value.naive_local()));
                    }
                }
                "bool" => {
                    let col_value: Option<bool> = row.get(col_name.as_str());
                    if let Some(value) = col_value {
                        orm_row
                            .get_data_mut()
                            .insert(col_name, RdbcValue::Boolean(value));
                    }
                }
                _ => {
                    tracing::warn!("postgres数据库暂未支持的列类型: {:#?}", col_type);
                    orm_row.get_data_mut().insert(col_name, RdbcValue::Null);
                }
            }
        }
        orm_row
    }
}
