use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::spawn_app;

#[tokio::test]
async fn subscribe_returns_200_for_valid_data() {
    let app = spawn_app().await;

    let sample_body = "name=s%20lem&email=solaris%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let response = app.post_subscriptions(sample_body.into()).await;

    assert_eq!(200, response.status().as_u16());

    let saved =
        sqlx::query!("SELECT name, email FROM subscriptions WHERE email='solaris@gmail.com'",)
            .fetch_one(&app.db_pool)
            .await
            .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, "solaris@gmail.com");
    assert_eq!(saved.name, "s lem");
}

#[tokio::test]
async fn subscribe_returns_400_when_no_data() {
    let app = spawn_app().await;

    let test_cases = vec![
        ("name=frank%20herbert", "missing email"),
        ("email=childrenofdune%40gmail.com", "missing name"),
        ("", "missing name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "Expected API to return 400 Bad Request for payload with {}",
            error_message
        );
    }
}

#[tokio::test]
async fn subscribe_returns_200_for_present_but_empty_fields() {
    let app = spawn_app().await;

    let test_cases = vec![
        ("name=&email=bob_the_builder%40gmail.com", "empty name"),
        ("name=bob&email=", "empty email"),
        ("name=bob&email=an_invalid_email_address", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = app.post_subscriptions(body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 200 OK when the payload was {}.",
            description
        )
    }
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    let app = spawn_app().await;
    let body = "name=s%20lem&email=solaris%40gmail.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;
}

#[tokio::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    
}