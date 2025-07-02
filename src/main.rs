mod client;
mod endpoint;
mod error;
mod models;
mod tools;

use crate::{client::ScalewayClient, tools::ScalewayTools};
use anyhow::Result;
use dotenvy;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};

/// npx @modelcontextprotocol/inspector cargo run
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    dotenvy::dotenv().ok();

    let secret_key =
        std::env::var("SCALEWAY_SECRET_KEY").expect("Env var SCALEWAY_SECRET_KEY not found");

    tracing::info!("SCALEWAY_SECRET_KEY {}", secret_key);

    tracing::info!("Starting MCP server");

    let api = ScalewayClient::new(&secret_key, None);

    let service = ScalewayTools::new(api)
        .serve(stdio())
        .await
        .inspect_err(|e| {
            tracing::error!("serving error: {:?}", e);
        })?;

    service.waiting().await?;

    Ok(())
}
