use newsletter::{
    application::get_conntection_pool,
    application::Application,
    configuration::{get_configuration, DatabaseSettings},
    email_client::EmailClient,
    telemetry::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;
use sqlx::{types::Uuid, Connection, Executor, PgConnection, PgPool};
use wiremock::MockServer;

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        let default_filter_level = "info".to_owned();
        let subscriber_name = "test".to_owned();
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    }
});
pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub api_client: reqwest::Client,
    pub db_pool: PgPool,
    pub email_client: EmailClient,
    pub email_server: MockServer,
}

pub struct ConfirmationLinks {
    pub html: reqwest::Url,
    pub plain_text: reqwest::Url,
}

impl TestApp {
    pub async fn post_subscriptions(&self, body: String) -> reqwest::Response {
        self.api_client
            .post(&format!("{}/subscriptions", &self.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub fn get_confirmation_links(&self, email_request: &wiremock::Request) -> ConfirmationLinks {
        let body: serde_json::Value = serde_json::from_slice(&email_request.body).unwrap();

        let get_link = |s: &str| {
            let links: Vec<_> = linkify::LinkFinder::new()
                .links(s)
                .filter(|l| *l.kind() == linkify::LinkKind::Url)
                .collect();
            assert_eq!(links.len(), 1);
            let raw_link = links[0].as_str().to_owned();
            let mut confirmation_link = reqwest::Url::parse(&raw_link).unwrap();
            // Let's make sure we don't call random APIs on the web.
            assert_eq!(confirmation_link.host_str().unwrap(), "127.0.0.1");
            confirmation_link.set_port(Some(self.port)).unwrap();
            confirmation_link
        };

        let html = get_link(body["HtmlBody"].as_str().unwrap());
        let plain_text = get_link(body["TextBody"].as_str().unwrap());
        ConfirmationLinks { html, plain_text }
    }
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let email_server = MockServer::start().await;

    let configuration = {
        let mut conf = get_configuration().expect("Failed to read configuration.");
        // Use random port.
        conf.application.port = 0;
        // Use a different database for each test case.
        conf.database.database_name = Uuid::new_v4().to_string();
        // Use the mock server as email API.
        conf.email_client.base_url = email_server.uri();
        conf
    };

    configure_database(&configuration.database).await;

    let client = reqwest::Client::new();

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    let _ = tokio::spawn(application.run());

    let test_app = TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
        api_client: client,
        db_pool: get_conntection_pool(&configuration.database),
        email_server,
        email_client: configuration.email_client.client(),
    };

    test_app
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to the database.");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    let db_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to the database.");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate the database.");

    db_pool
}
