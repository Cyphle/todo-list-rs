
#[cfg(test)]
mod tests {
    mod todo_lists_read {
        use sea_orm::{DatabaseBackend, DbErr, MockDatabase, Transaction};
        use sea_orm::{EntityTrait};

        use entity::todo_lists::Entity as TodoLists;

        #[async_std::test]
        async fn test() -> Result<(), DbErr> {
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

        #[async_std::test]
        async fn test_insert_todo_lists() -> Result<(), DbErr> {
            // Create MockDatabase with mock execution result
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
            let apple = entity::todo_lists::ActiveModel {
                title: Set("Apple Pie".to_owned()),
                ..Default::default()
            };

            // Insert the ActiveModel into MockDatabase
            let inserted = apple.clone().insert(&db).await?;
            assert_eq!(
                inserted,
                entity::todo_lists::Model {
                    id: 15,
                    title: "Apple Pie".to_owned()
                }
            );

            // Checking transaction log
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