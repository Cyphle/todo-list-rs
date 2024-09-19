
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

    // mod todo_lists_write {
    //     use sea_orm::{
    //         entity::prelude::*, entity::*, tests_cfg::*,
    //         DatabaseBackend, MockDatabase, MockExecResult, Transaction,
    //     };
    //
    //     #[async_std::test]
    //     async fn test_insert_cake() -> Result<(), DbErr> {
    //         // Create MockDatabase with mock execution result
    //         let db = MockDatabase::new(DatabaseBackend::Postgres)
    //             .append_query_results([
    //                 [cake::Model {
    //                     id: 15,
    //                     name: "Apple Pie".to_owned(),
    //                 }],
    //                 [cake::Model {
    //                     id: 16,
    //                     name: "Apple Pie".to_owned(),
    //                 }],
    //             ])
    //             .append_exec_results([
    //                 MockExecResult {
    //                     last_insert_id: 15,
    //                     rows_affected: 1,
    //                 },
    //                 MockExecResult {
    //                     last_insert_id: 16,
    //                     rows_affected: 1,
    //                 },
    //             ])
    //             .into_connection();
    //
    //         // Prepare the ActiveModel
    //         let apple = cake::ActiveModel {
    //             name: Set("Apple Pie".to_owned()),
    //             ..Default::default()
    //         };
    //
    //         // Insert the ActiveModel into MockDatabase
    //         assert_eq!(
    //             apple.clone().insert(&db).await?,
    //             cake::Model {
    //                 id: 15,
    //                 name: "Apple Pie".to_owned()
    //             }
    //         );
    //
    //         // If you want to check the last insert id
    //         let insert_result = cake::Entity::insert(apple).exec(&db).await?;
    //         assert_eq!(insert_result.last_insert_id, 16);
    //
    //         // Checking transaction log
    //         assert_eq!(
    //             db.into_transaction_log(),
    //             [
    //                 Transaction::from_sql_and_values(
    //                     DatabaseBackend::Postgres,
    //                     r#"INSERT INTO "cake" ("name") VALUES ($1) RETURNING "id", "name""#,
    //                     ["Apple Pie".into()]
    //                 ),
    //                 Transaction::from_sql_and_values(
    //                     DatabaseBackend::Postgres,
    //                     r#"INSERT INTO "cake" ("name") VALUES ($1) RETURNING "id""#,
    //                     ["Apple Pie".into()]
    //                 ),
    //             ]
    //         );
    //
    //         Ok(())
    //     }
    // }
}