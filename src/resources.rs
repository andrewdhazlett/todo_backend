use actix_web::{error, HttpRequest, Json, Path, Responder, State};
use app::{NewTodo, Todo, TodoState};
use uuid::{Uuid, UuidVersion};

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
    let todos = state.todos.borrow();

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
        let url = req.url_for("todo", vec![&id])
            .expect("expected it to make a url for the todo")
            .to_string();
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
            state.todos.borrow_mut().push(todo.clone());
            Ok(Json(todo))
        }
        None => Err(error::ErrorBadRequest("unable to create todo")),
    }
}

pub fn delete_todos(state: State<TodoState>) -> impl Responder {
    let todos = Vec::new();
    state.todos.replace(todos.clone());
    Json(todos)
}
