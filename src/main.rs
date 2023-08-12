use newsletter::configuration::get_configuration;
use newsletter::startup::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration file");

    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to database");

    let server_address = format!("127.0.0.1:{}", configuration.application_port);

    let listener =
        TcpListener::bind(server_address).expect("Could not bind system assigned address");

    run(listener, connection_pool)?.await
}
