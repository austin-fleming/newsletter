// cargo expand --test health_check

use std::net::TcpListener;

#[tokio::test]
async fn ping_endpoint_works() {
    let server_address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/ping", server_address))
        .send()
        .await
        .expect("Request failed to execute");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind system assigned port");
    let port = listener.local_addr().expect("Could not retrieve port number from listener").port();

    let server = newsletter::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
