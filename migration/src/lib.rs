pub use sea_orm_migration::prelude::*;

mod m20240808_142724_create_todo_list;
mod m20240808_151647_create_todo_list_items;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240808_142724_create_todo_list::Migration),
            Box::new(m20240808_151647_create_todo_list_items::Migration),
        ]
    }
}
