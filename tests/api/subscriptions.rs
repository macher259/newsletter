use wiremock::{
    matchers::{method, path},
    Mock, ResponseTemplate,
};

use crate::helpers::spawn_app;

#[actix_web::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;
    let body = "name=Maciej&email=Maciej@domain.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    let response = app.post_subscriptions(body.to_owned()).await;

    assert_eq!(200, response.status().as_u16());
}

#[actix_web::test]
async fn subscribe_persists_the_new_subscriber() {
    let app = spawn_app().await;
    let body = "name=Maciej&email=Maciej@domain.com";

    app.post_subscriptions(body.to_owned()).await;

    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "Maciej@domain.com");
    assert_eq!(saved.name, "Maciej");
    assert_eq!(saved.status, "pending confirmation");
}

#[actix_web::test]
async fn subscribe_fails_on_database_error() {
    let app = spawn_app().await;
    let body = "name=Maciej&email=Maciej@domain.com";

    sqlx::query!("ALTER TABLE subscriptions DROP COLUMN email;")
        .execute(&app.db_pool)
        .await
        .unwrap();

    let response = app.post_subscriptions(body.to_owned()).await;
    assert_eq!(response.status().as_u16(), 500);
}

#[actix_web::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=Maciej", "missing email"),
        ("email=Maciej@domain.com", "missing name"),
        ("", "missing email and missing name"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.to_owned()).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "Subscribe didn't return 400 when the payload was {}.",
            error_message
        );
    }
}

#[actix_web::test]
async fn subscribe_returns_a_400_when_data_is_invalid() {
    let app = spawn_app().await;
    let test_cases = vec![
        ("name=Maciej&email=", "empty email"),
        ("email=Maciej@domain.com&name=", "empty name"),
        ("name=Maciej&email=Maciej.com", "invalid email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.to_owned()).await;
        assert_eq!(
            400,
            response.status().as_u16(),
            "Subscribe didn't return 400 when the payload was {}.",
            error_message
        );
    }
}

#[actix_web::test]
async fn subscribe_sends_a_confirmation_email_with_a_link() {
    let app = spawn_app().await;
    let body = "name=Maciej&email=Maciej@domain.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;

    let email_request = &app.email_server.received_requests().await.unwrap()[0];
    let confirmation_links = app.get_confirmation_links(email_request);

    assert_eq!(confirmation_links.html, confirmation_links.plain_text);
}

#[actix_web::test]
async fn subscribe_sends_a_confirmation_email_for_valid_data() {
    let app = spawn_app().await;
    let body = "name=Maciej&email=Maciej@domain.com";

    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;

    app.post_subscriptions(body.into()).await;
}
