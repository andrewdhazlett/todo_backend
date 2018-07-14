extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

mod app;
mod resources;

use actix::System;
use actix_web::server;
use app::create_app;

fn main() {
    let sys = System::new("todo_backend");
    server::new(create_app)
        .bind("localhost:3000")
        .unwrap()
        .start();
    sys.run();
}
