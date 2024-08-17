use std::net::TcpListener;

use actix_web::{HttpRequest, Responder};
use secrecy::ExposeSecret;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use zero2prod::{
    configuration::get_configuration,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5) // Set the maximum number of connections in the pool
        .connect(&configuration.database.connection_string().expose_secret())
        .await
        .expect("Failed to connect");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address).expect("Failed to bind random port");

    run(listener, pool)?.await
}

#[allow(dead_code)]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
