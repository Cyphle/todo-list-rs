use sea_orm_migration::{prelude::*, schema::*};
use sea_orm::Statement;
use crate::setup::{DOWN_SQL_DIR, execute_sql, read_sql_file, UP_SQL_DIR};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Use `execute_unprepared` if the SQL statement doesn't have value bindings
        let create_todo_list_sql = read_sql_file(format!("{}01-create-todo-lists-table.sql", UP_SQL_DIR))?;
        execute_sql(db, &create_todo_list_sql).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let drop_todo_list_sql = read_sql_file(format!("{}01-drop-todo-lists-table.sql", DOWN_SQL_DIR))?;
        execute_sql(db, &drop_todo_list_sql).await?;

        Ok(())
    }
}

