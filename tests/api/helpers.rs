use newsletter::{application::Application, configuration::get_configuration};

pub struct TestApp {
    pub address: String,
    pub port: u16,
}

pub async fn spawn_app() -> TestApp {
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
