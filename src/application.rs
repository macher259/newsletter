use std::net::TcpListener;

use actix_web::{dev::Server, App, HttpServer};

use crate::configuration::Settings;
use crate::routes::healthcheck;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(listener).await?;

        Ok(Self { port, server })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub const fn port(&self) -> u16 {
        self.port
    }
}

async fn run(listener: TcpListener) -> Result<Server, anyhow::Error> {
    let server = HttpServer::new(|| App::new().service(healthcheck))
        .listen(listener)?
        .run();

    Ok(server)
}
