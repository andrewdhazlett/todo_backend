extern crate actix;
extern crate actix_web;

use actix::{System};
use actix_web::{App, server};

fn main() {
    let sys = System::new("todo_backend");
    server::new(|| App::new())
        .bind("localhost:3000")
        .unwrap()
        .start();
    sys.run();
}
