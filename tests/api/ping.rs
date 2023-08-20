use crate::helpers::spawn_app;

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
