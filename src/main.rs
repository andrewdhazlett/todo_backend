extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate serde_derive;

use actix::System;
use actix_web::{http, server, App, Path, State};
use std::cell::RefCell;

struct TodoState {
    name: RefCell<String>,
}

#[derive(Deserialize)]
struct GreetingParams {
    name: String,
}

fn main() {
    let sys = System::new("todo_backend");
    server::new(|| {
        let state = TodoState {
            name: RefCell::from("Fred".to_string()),
        };
        App::with_state(state)
            .resource("/", |req| {
                req.method(http::Method::GET)
                    .with(|state: State<TodoState>| {
                        let name = state.name.try_borrow().unwrap();
                        format!("Hello, {}!", &name)
                    })
            })
            .resource("/{name}", |req| {
                req.method(http::Method::PUT).with(
                    |(state, params): (
                        State<TodoState>,
                        Path<GreetingParams>,
                    )| {
                        state.name.replace(params.name.clone());
                        "success"
                    },
                );
            })
    }).bind("localhost:3000")
        .unwrap()
        .start();
    sys.run();
}
