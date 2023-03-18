use newsletter::{application::Application, configuration::get_configuration};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run().await?;

    Ok(())
}
