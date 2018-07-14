use actix_web::{Json, State};
use app::{Todo, TodoState};

pub fn get_todos(state: State<TodoState>) -> Json<Vec<Todo>> {
    Json(state.todos.clone())
}
