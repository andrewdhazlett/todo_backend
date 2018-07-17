use actix_web::{error, HttpRequest, Json, Path, Responder, State};
use app::{NewTodo, Todo, TodoState};
use std::sync::Arc;
use uuid::{Uuid, UuidVersion};

pub fn get_todos(state: State<TodoState>) -> Json<Vec<Todo>> {
    let todos = Arc::clone(&state.todos);
    let todos = todos.lock().unwrap().to_vec();
    Json(todos)
}

#[derive(Deserialize)]
pub struct GetTodoParams {
    id: String,
}

pub fn get_todo(
    (state, params): (State<TodoState>, Path<GetTodoParams>),
) -> Result<Json<Todo>, error::Error> {
    let id = params.id.clone();
    let todos = Arc::clone(&state.todos);
    let todos = todos.lock().unwrap();

    match todos.iter().find(|&todo| todo.id == id) {
        Some(todo) => Ok(Json(todo.clone())),
        None => Err(error::ErrorNotFound(id)),
    }
}

pub fn create_todo(
    (state, body, req): (
        State<TodoState>,
        Json<NewTodo>,
        HttpRequest<TodoState>,
    ),
) -> Result<Json<Todo>, error::Error> {
    let todo = Uuid::new(UuidVersion::Sha1).map(|uuid| {
        let id = uuid.to_string();
        let url = req.url_for("todo", vec![&id]).unwrap().to_string();
        Todo {
            id,
            url,
            title: body.title.clone(),
            order: 0,
            completed: false,
        }
    });

    match todo {
        Some(todo) => {
            let todos = Arc::clone(&state.todos);
            todos.lock().unwrap().push(todo.clone());
            Ok(Json(todo))
        }
        None => Err(error::ErrorBadRequest("unable to create todo")),
    }
}

pub fn delete_todos(state: State<TodoState>) -> impl Responder {
    let todos = Arc::clone(&state.todos);
    let todos = todos.lock().unwrap().truncate(0);
    Json(todos)
}
