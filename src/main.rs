extern crate actix;
extern crate actix_web;

use actix::System;
use actix_web::{http, server, App, HttpRequest, HttpResponse};

fn main() {
    let sys = System::new("todo_backend");
    server::new(|| {
        App::new().route(
            "/",
            http::Method::GET,
            |_: HttpRequest| -> HttpResponse {
                HttpResponse::Ok().body("Hello, world!")
            },
        )
    }).bind("localhost:3000")
        .unwrap()
        .start();
    sys.run();
}
