mod sea_orm;
use sea_orm::{DatabaseConnection, DbErr, ActiveModelTrait, EntityTrait};
use entity::todo_lists;
use sea_orm::ActiveValue::Set;
use crate::models::todo::TodoList;

pub struct TodoListRepository {
    db: DatabaseConnection,
}

impl TodoListRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        TodoListRepository { db }
    }

    pub async fn save(&self, todo: TodoList) -> Result<(), DbErr> {
        let new_todo = todo_lists::ActiveModel {
            id: Default::default(),
            title: Set(todo.title),
        };

        new_todo.insert(&self.db).await?;
        Ok(())
    }

    pub async fn find_all(&self) -> Result<Vec<todo_lists::Model>, DbErr> {
        todo_lists::Entity::find().all(&self.db).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<todo_lists::Model>, DbErr> {
        todo_lists::Entity::find_by_id(id).one(&self.db).await
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::{
        entity::prelude::*, entity::*, tests_cfg::*,
        DatabaseBackend, MockDatabase, Transaction,
    };

    #[async_std::test]
    async fn test_find_cake() -> Result<(), DbErr> {
        // Create MockDatabase with mock query results
        let db: &DatabaseConnection = &MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([
                // First query result
                vec![bakery::Model {
                    id: 1,
                    name: "Happy Bakery".to_owned(),
                    profit_margin: 0.0,
                }],
            ])
            .into_connection();

        // Find a cake from MockDatabase
        // Return the first query result
        assert_eq!(
            cake::Entity::find().one(&db).await?,
            Some(cake::Model {
                id: 1,
                name: "New York Cheese".to_owned(),
            })
        );

        // Checking transaction log
        assert_eq!(
            db.into_transaction_log(),
            [
                Transaction::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    r#"SELECT "cake"."id", "cake"."name" FROM "cake" LIMIT $1"#,
                    [1u64.into()]
                )
            ]
        );

        Ok(())
    }
}