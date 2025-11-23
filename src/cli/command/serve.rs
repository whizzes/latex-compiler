use std::net::{IpAddr, SocketAddr};

use anyhow::{Context, Result};
use axum::serve;
use clap::Args;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::router::make_router;

#[derive(Args, Clone, Debug)]
pub struct ServeCmd {
    #[clap(long = "host", env = "HOST", default_value = "0.0.0.0")]
    pub host: IpAddr,
    #[clap(long = "port", env = "PORT", default_value = "9000")]
    pub port: u16,
}

impl ServeCmd {
    pub async fn exec(&self) -> Result<()> {
        let filter_layer =
            EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer())
            .with(filter_layer)
            .init();

        let app = make_router();
        let addr = SocketAddr::from((self.host, self.port));

        info!("Server listening on {}", addr);
        info!("OpenAPI documentation available at {}/swagger-ui/", addr);

        let listener = TcpListener::bind(addr)
            .await
            .context("Failed to bind to address")?;

        serve(listener, app).await?;
        Ok(())
    }
}
