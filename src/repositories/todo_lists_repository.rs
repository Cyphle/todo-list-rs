
#[cfg(test)]
mod tests {
    use sea_orm::{DatabaseBackend, DbErr, MockDatabase};
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
        // assert_eq!(
        //     db.into_transaction_log(),
        //     [
        //         Transaction::from_sql_and_values(
        //             DatabaseBackend::Postgres,
        //             r#"SELECT "cake"."id", "cake"."name" FROM "cake" LIMIT $1"#,
        //             [1u64.into()]
        //         ),
        //         Transaction::from_sql_and_values(
        //             DatabaseBackend::Postgres,
        //             r#"SELECT "cake"."id", "cake"."name" FROM "cake""#,
        //             []
        //         ),
        //         Transaction::from_sql_and_values(
        //             DatabaseBackend::Postgres,
        //             r#"SELECT "cake"."id" AS "A_id", "cake"."name" AS "A_name", "fruit"."id" AS "B_id", "fruit"."name" AS "B_name", "fruit"."cake_id" AS "B_cake_id" FROM "cake" LEFT JOIN "fruit" ON "cake"."id" = "fruit"."cake_id""#,
        //             []
        //         ),
        //     ]
        // );

        Ok(())
    }
}