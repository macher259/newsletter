use newsletter::{
    application::Application,
    configuration::get_configuration,
    telemetry::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;

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
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let configuration = {
        let mut conf = get_configuration().expect("Failed to read configuration.");
        // Use random port.
        conf.application.port = 0;
        conf
    };

    let application = Application::build(configuration)
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    let _ = tokio::spawn(application.run());

    let test_app = TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
    };

    test_app
}
