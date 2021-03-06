use actix_web::{http::Method, middleware::cors::Cors, App};
use resources::*;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub order: u64,
    pub completed: bool,
    pub url: String,
}

#[derive(Deserialize)]
pub struct NewTodo {
    pub title: String,
}

pub struct TodoState {
    pub todos: Arc<Mutex<Vec<Todo>>>,
}

pub fn create_app() -> App<TodoState> {
    let state = TodoState {
        todos: Arc::from(Mutex::from(vec![Todo {
            id: "default todo id".to_string(),
            title: "Learn redux".to_string(),
            order: 0,
            completed: false,
            url: "".to_string(),
        }])),
    };
    App::with_state(state).configure(|app| {
        Cors::for_app(app)
            .resource("/", |r| {
                r.method(Method::GET).with(get_todos);
                r.method(Method::POST).with(create_todo);
                r.method(Method::DELETE).with(delete_todos);
            })
            .resource("/{id}", |r| {
                r.name("todo");
                r.method(Method::GET).with(get_todo);
            })
            .register()
    })
}
