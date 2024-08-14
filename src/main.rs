use std::net::TcpListener;

use actix_web::{HttpRequest, Responder};
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration =
        zero2prod::configuration::get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address).expect("Failed to bind random port");
    run(listener)?.await
}

#[allow(dead_code)]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
