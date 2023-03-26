use std::net::TcpListener;

use actix_web::web::Data;
use actix_web::{dev::Server, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::configuration::{DatabaseSettings, Settings};
use crate::email_client::EmailClient;
use crate::routes::{healthcheck, home, subscribe};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let db_pool = get_conntection_pool(&configuration.database);
        sqlx::migrate!("./migrations").run(&db_pool).await?;

        let email_client = configuration.email_client.client();

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
            db_pool,
            email_client,
            configuration.application.base_url,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub const fn port(&self) -> u16 {
        self.port
    }
}

pub fn get_conntection_pool(configuration: &DatabaseSettings) -> PgPool {
    let timeout = std::time::Duration::from_secs(2);
    PgPoolOptions::new()
        .acquire_timeout(timeout)
        .connect_lazy_with(configuration.with_db())
}

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
    email_client: EmailClient,
    base_url: String,
) -> Result<Server, anyhow::Error> {
    let db_pool = Data::new(db_pool);
    let email_client = Data::new(email_client);
    let base_url = Data::new(base_url);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(healthcheck)
            .service(home)
            .service(subscribe)
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
