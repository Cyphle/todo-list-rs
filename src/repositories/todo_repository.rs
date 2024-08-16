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