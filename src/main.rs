use newsletter::{
    application::Application,
    configuration::get_configuration,
    telemetry::{get_subscriber, init_subscriber},
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("app".to_owned(), "info".to_owned(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run().await?;

    Ok(())
}
