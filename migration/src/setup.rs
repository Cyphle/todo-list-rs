use sea_orm_migration::prelude::*;
use std::fs;
use std::path::Path;
use crate::sea_orm::ExecResult;

pub const UP_SQL_DIR: &str = "migration/sql/up";
pub const DOWN_SQL_DIR: &str = "migration/sql/down";

pub fn read_sql_file<P: AsRef<Path>>(path: P) -> Result<String, DbErr> {
    fs::read_to_string(path)
        .map_err(|err| DbErr::Custom(format!("Failed to read file: {:?}", err)))
}

pub async fn execute_sql(db: &SchemaManagerConnection<'_>, sql: &str) -> Result<ExecResult, DbErr> {
    log::info!("Executing SQL: {}", sql);
    db.execute_unprepared(sql).await
}