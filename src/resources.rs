use actix_web::{Json, State};
use app::{NewTodo, Todo, TodoState};
use uuid::Uuid;

pub fn get_todos(state: State<TodoState>) -> Json<Vec<Todo>> {
    Json(state.todos.clone())
}

pub fn create_todo(body: Json<NewTodo>) -> Json<Todo> {
    Json(Todo {
        id: Uuid::nil().to_string(),
        title: body.title.clone(),
        order: 0,
        completed: false,
        url: "".to_string(),
    })
}
