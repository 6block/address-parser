use std::{net::SocketAddr, time::Duration};

use address_parser::{handler_404, initialize_logger, parse_handler};
use anyhow::Result;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, BoxError, Router};
use clap::Parser;
use tokio::net::TcpListener;
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

#[derive(Debug, Parser)]
struct AddressParser {
    /// The ip:port listening for incoming requests.
    #[clap(long)]
    listen: SocketAddr,
    /// Specify the verbosity of the server [options: 0, 1, 2].
    #[clap(short, long, default_value = "0")]
    verbosity: u8,
}

impl AddressParser {
    async fn start(&self) -> Result<()> {
        let router = Router::new()
            .route("/parse", get(parse_handler))
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(|_: BoxError| async {
                        StatusCode::REQUEST_TIMEOUT
                    }))
                    .layer(TimeoutLayer::new(Duration::from_secs(10))),
            )
            .layer(
                CorsLayer::new()
                    .allow_methods(Any)
                    .allow_origin(Any)
                    .allow_headers(Any),
            );

        let listener = TcpListener::bind(self.listen).await?;
        let app = router.fallback(handler_404);
        info!("Server listening on {}", self.listen);
        axum::serve(listener, app).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let parser = AddressParser::parse();
    initialize_logger(parser.verbosity);
    info!("Start address parser {:?}", parser);
    parser.start().await?;
    Ok(())
}
