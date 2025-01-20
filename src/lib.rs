mod tokens;
pub use tokens::*;

use anyhow::anyhow;
use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::trace;
use tracing_subscriber::EnvFilter;

pub fn initialize_logger(verbosity: u8) {
    match verbosity {
        0 => std::env::set_var("RUST_LOG", "info"),
        1 => std::env::set_var("RUST_LOG", "debug"),
        2 | 3 | 4 => std::env::set_var("RUST_LOG", "trace"),
        _ => std::env::set_var("RUST_LOG", "info"),
    };
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressQuery {
    pub token: String,
    pub address: String,
}

pub async fn parse_handler(Query(params): Query<AddressQuery>) -> impl IntoResponse {
    trace!("new parse request: {:?}", params);
    let parse_result = match params.token.as_str() {
        Aleo::NAME => is_valid_aleo_address(params.address),
        Autonomys::NAME => is_valid_auto_address(params.address),
        Ironfish::NAME => is_valid_iron_address(params.address),
        _ => Err(anyhow!("Unknown token used")),
    };

    match parse_result {
        Ok(_) => Json(json!({"code": 200, "data": true})).into_response(),
        Err(e) => Json(json!({"code": 601, "error": e.to_string()})).into_response(),
    }
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found").into_response()
}
