use actix_web::{Json, State};
use app::{NewTodo, Todo, TodoState};
use uuid::Uuid;

pub fn get_todos(state: State<TodoState>) -> Json<Vec<Todo>> {
    Json(state.todos.borrow().clone())
}

pub fn create_todo((state, body): (State<TodoState>, Json<NewTodo>)) -> Json<Todo> {
    let todo = Todo {
        id: Uuid::nil().to_string(),
        title: body.title.clone(),
        order: 0,
        completed: false,
        url: "".to_string(),
    };
    state.todos.borrow_mut().push(todo.clone());
    Json(todo)
}

pub fn delete_todos(state: State<TodoState>) -> Json<Vec<Todo>> {
    let todos = Vec::new();
    state.todos.replace(todos.clone());
    Json(todos)
}
