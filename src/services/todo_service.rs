use crate::models::todo::{TodoList, TodoListView};
use crate::repositories::todo_repository::TodoListRepository;

pub struct TodoListService {
    repository: TodoListRepository,
}

// TODO to be deleted. stop l'objet quoi. C'est pour utiliser les AppState extractor d'actix. A voir l'utilité...
impl TodoListService {
    pub fn new(repository: TodoListRepository) -> Self {
        TodoListService { repository }
    }

    pub async fn create_todo(&self, dto: TodoListView) -> Result<(), ()> {
        let new_todo = TodoList::new(dto);
        self.repository.save(new_todo).await.map_err(|_| ())
    }

    pub async fn get_all_todos(&self) -> Result<Vec<TodoListView>, ()> {
        match self.repository.find_all().await {
            Ok(todos) => Ok(todos.into_iter().map(|t| TodoListView::new(t)).collect()),
            Err(_) => Err(()),
        }
    }

    pub async fn get_todo_by_id(&self, id: i32) -> Result<Option<TodoListView>, ()> {
        match self.repository.find_by_id(id).await {
            Ok(Some(todo)) => Ok(Some(TodoListView::new(todo))),
            Ok(None) => Ok(None),
            Err(_) => Err(()),
        }
    }
}