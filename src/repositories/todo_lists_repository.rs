use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};
use crate::domain::todo_list::{CreateTodoListCommand, TodoList};

pub async fn create(db_connexion: &DatabaseConnection, command: CreateTodoListCommand) -> Result<TodoList, DbErr> {
    let model = entity::todo_lists::ActiveModel {
        title: Set(command.title.to_owned()),
        ..Default::default()
    };

    model.clone().insert(db_connexion).await.map(|m| TodoList {
        id: m.id,
        title: m.title
    })
}

#[cfg(test)]
mod tests {
    mod todo_lists_read {
        use sea_orm::{DatabaseBackend, DbErr, MockDatabase, Transaction};
        use sea_orm::{EntityTrait};

        use entity::todo_lists::Entity as TodoLists;

        #[async_std::test]
        async fn test_example() -> Result<(), DbErr> {
            // Create MockDatabase with mock query results
            let db = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results([
                    // First query result
                    vec![entity::todo_lists::Model {
                        id: 1,
                        title: "New York Cheese".to_owned(),
                    }],
                ])
                .into_connection();

            // Find a cake from MockDatabase
            // Return the first query result
            assert_eq!(
                TodoLists::find_by_id(1).one(&db).await?,
                Some(entity::todo_lists::Model {
                    id: 1,
                    title: "New York Cheese".to_owned(),
                })
            );

            // Checking transaction log
            assert_eq!(
                db.into_transaction_log(),
                [
                    Transaction::from_sql_and_values(
                        DatabaseBackend::Postgres,
                        r#"SELECT "todo_lists"."id", "todo_lists"."title" FROM "todo_lists" WHERE "todo_lists"."id" = $1 LIMIT $2"#,
                        [1.into(), 1u64.into()],
                    ),
                ]
            );

            Ok(())
        }
    }

    mod todo_lists_write {
        use sea_orm::{
            entity::prelude::*, entity::*,
            DatabaseBackend, MockDatabase, MockExecResult, Transaction,
        };
        use crate::domain::todo_list::{CreateTodoListCommand, TodoList};
        use crate::repositories::todo_lists_repository::create;

        #[async_std::test]
        async fn should_create_todo_list() -> Result<(), DbErr> {
            let db = MockDatabase::new(DatabaseBackend::Postgres)
                .append_query_results([
                    [entity::todo_lists::Model {
                        id: 15,
                        title: "Apple Pie".to_owned(),
                    }],
                ])
                .append_exec_results([
                    MockExecResult {
                        last_insert_id: 15,
                        rows_affected: 1,
                    },
                ])
                .into_connection();

            // Prepare the ActiveModel
            let created = create(&db, CreateTodoListCommand {
                title: "Apple Pie".to_string()
            }).await;

            assert_eq!(
                created.unwrap(),
                TodoList {
                    id: 15,
                    title: "Apple Pie".to_owned()
                }
            );

            assert_eq!(
                db.into_transaction_log(),
                [
                    Transaction::from_sql_and_values(
                        DatabaseBackend::Postgres,
                        r#"INSERT INTO "todo_lists" ("title") VALUES ($1) RETURNING "id", "title""#,
                        ["Apple Pie".into()]
                    ),
                ]
            );

            Ok(())
        }
    }
}