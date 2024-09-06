use sea_orm::{ActiveModelTrait, Database, DatabaseConnection, DbErr, EntityTrait};

pub fn connect() -> Result<DatabaseConnection, DbErr> {
    Database::connect("postgres://postgres:postgres@localhost:5434/todolist").await
}