use actix_web::{error, Json, Path, Responder, State};
use app::{NewTodo, Todo, TodoState};
use uuid::Uuid;

pub fn get_todos(state: State<TodoState>) -> Json<Vec<Todo>> {
    Json(state.todos.borrow().clone())
}

#[derive(Deserialize)]
pub struct GetTodoParams {
    id: String,
}

pub fn get_todo(
    (state, params): (State<TodoState>, Path<GetTodoParams>),
) -> Result<Json<Todo>, error::Error> {
    let id = params.id.clone();
    let todos = state.todos.borrow_mut();
    todos.iter().find(|&todo| todo.id == id).map_or_else(
        || Err(error::ErrorNotFound(id)),
        |todo| Ok(Json(todo.clone())),
    )
}

pub fn create_todo(
    (state, body): (State<TodoState>, Json<NewTodo>),
) -> impl Responder {
    let id = Uuid::nil().to_string();
    let todo = Todo {
        id: id.clone(),
        title: body.title.clone(),
        order: 0,
        completed: false,
        url: format!("./todos/{}", id),
    };
    state.todos.borrow_mut().push(todo.clone());
    Json(todo)
}

pub fn delete_todos(state: State<TodoState>) -> impl Responder {
    let todos = Vec::new();
    state.todos.replace(todos.clone());
    Json(todos)
}
