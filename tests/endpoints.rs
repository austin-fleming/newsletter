// cargo expand --test health_check

use newsletter::configuration::{get_configuration, DatabaseSettings};
use newsletter::startup;
use sqlx::{PgPool, PgConnection, Connection, Executor};
use uuid::Uuid;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect(
        &config.connection_string_without_db()
    )
    .await
    .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database.");

    connection_pool
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind system assigned port");
    let port = listener
        .local_addr()
        .expect("Could not retrieve port number from listener")
        .port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let db_pool = configure_database(&configuration.database)
        .await;

    let server = startup::run(listener, db_pool.clone()).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp { address, db_pool }
}

#[tokio::test]
async fn ping_endpoint_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/ping", app.address))
        .send()
        .await
        .expect("Request failed to execute");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let sample_body = "name=frank%20herbert&email=dunemessiah%40gmail.com";

    let response = client
        .post(format!("{}/subscriptions", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(sample_body)
        .send()
        .await
        .expect("Request failed to execute");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT name, email FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "dunemessiah@gmail.com");
    assert_eq!(saved.name, "frank herbert");
}

#[tokio::test]
async fn subscribe_returns_400_when_no_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=frank%20herbert", "missing email"),
        ("email=dunemessiah%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Request failed to execute");

        assert_eq!(
            400,
            response.status().as_u16(),
            "Expected API to return 400 Bad Request for payload with {}",
            error_message
        );
    }
}
