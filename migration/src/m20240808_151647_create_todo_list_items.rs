use sea_orm_migration::{prelude::*};
use crate::setup::{DOWN_SQL_DIR, UP_SQL_DIR, execute_sql_scripts};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let file_names = vec![
            "02-create-todo-list-items-table.sql",
            "03-add-foreign-key-to-todo-list-items.sql",
        ];

        if let Err(err) = execute_sql_scripts(db, UP_SQL_DIR, &file_names).await {
            return Err(err);
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let file_names = vec![
            "03-drop-foreign-key-in-todo-list-items.sql",
            "02-drop-todo-list-items-table.sql"
        ];

        if let Err(err) = execute_sql_scripts(db, DOWN_SQL_DIR, &file_names).await {
            return Err(err);
        }

        Ok(())
    }
}
