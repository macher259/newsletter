use actix_web::rt::task::JoinError;
use actix_web_lab::__reexports::tokio::select;
use newsletter::{
    application::Application,
    configuration::get_configuration,
    issue_delivery_worker::run_worker,
    telemetry::{get_subscriber, init_subscriber},
};
use std::fmt::{Debug, Display};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("app".to_owned(), "info".to_owned(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone()).await?;

    let application_task = actix_web::rt::spawn(application.run());
    let worker_task = actix_web::rt::spawn(run_worker(configuration));

    select! {
        x = application_task => report_exit("API", x),
        x = worker_task =>  report_exit("Background worker", x),
    }

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
