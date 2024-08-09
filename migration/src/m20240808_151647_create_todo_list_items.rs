use sea_orm_migration::{prelude::*, schema::*};
use crate::setup::{DOWN_SQL_DIR, execute_sql, read_sql_file, UP_SQL_DIR};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let create_todo_list_items_sql = read_sql_file(format!("{}02-create-todo-list-items-table.sql", UP_SQL_DIR))?;
        execute_sql(db, &create_todo_list_items_sql).await?;

        let add_foreign_key_sql = read_sql_file(format!("{}03-add-foreign-key-todo-list-items.sql", UP_SQL_DIR))?;
        execute_sql(db, &add_foreign_key_sql).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let drop_foreign_key_sql = read_sql_file(format!("{}03-drop-foreign-key-todo-list-items.sql", DOWN_SQL_DIR))?;
        execute_sql(db, &drop_foreign_key_sql).await?;

        let drop_todo_list_items_sql = read_sql_file(format!("{}02-drop-todo-list-items-table.sql", DOWN_SQL_DIR))?;
        execute_sql(db, &drop_todo_list_items_sql).await?;

        Ok(())
    }
}
