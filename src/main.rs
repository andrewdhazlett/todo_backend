extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate serde_derive;

use actix::System;
use actix_web::{http, server, App, Path, Result};

#[derive(Deserialize)]
struct GreetingParams {
    name: String,
}

fn main() {
    let sys = System::new("todo_backend");
    server::new(|| {
        App::new().resource("/{name}", |req| {
            req.method(http::Method::GET).with(
                |params: Path<GreetingParams>| -> Result<String> {
                    Ok(format!("Hello, {}", params.name))
                },
            )
        })
    }).bind("localhost:3000")
        .unwrap()
        .start();
    sys.run();
}
