use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    Database::connect("postgres://postgres:postgres@localhost:5434/todolist").await
}