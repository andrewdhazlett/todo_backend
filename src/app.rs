use actix_web::{http::Method, App};
use resources::{get_greeting, put_name};
use std::cell::RefCell;

pub struct TodoState {
    pub name: RefCell<String>,
}

pub fn create_app() -> App<TodoState> {
    let state = TodoState {
        name: RefCell::from("Fred".to_string()),
    };
    App::with_state(state)
        .resource("/", |r| r.method(Method::GET).with(get_greeting))
        .resource("/{name}", |r| r.method(Method::PUT).with(put_name))
}
