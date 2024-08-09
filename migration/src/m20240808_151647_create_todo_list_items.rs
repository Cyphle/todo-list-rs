use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE TABLE IF NOT EXISTS todo_list_items(
                    id      SERIAL PRIMARY KEY,
                    content TEXT
                 );"
        )
            .await?;

        db.execute_unprepared(
            "ALTER TABLE IF EXISTS todo_list_items ADD COLUMN todo_list_id INTEGER NOT NULL REFERENCES todo_lists(id);"
        )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE IF EXISTS todo_list_items DROP COLUMN IF EXISTS todo_list_id;")
            .await?;
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS todo_list_items")
            .await?;

        Ok(())
    }
}
