use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use prometheus_client::{encoding::text::encode, registry::Registry};
use reqwest::{StatusCode, header::CONTENT_TYPE};
use tracing::instrument;
use twox_hash::XxHash3_64;

use crate::env::METRICS_SECRET_HASH;

#[instrument(skip(registry, bearer))]
pub async fn metrics(
    State(registry): State<Arc<Registry>>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Response {
    let token_hash = XxHash3_64::oneshot(bearer.token().as_bytes());
    let token_hash_hex_str = format!("{token_hash:016x}");
    if token_hash_hex_str != *METRICS_SECRET_HASH {
        return StatusCode::FORBIDDEN.into_response();
    }

    let mut buffer = String::new();
    let Ok(()) = encode(&mut buffer, &registry) else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let res = Response::builder()
        .header(
            CONTENT_TYPE,
            "application/openmetrics-text; version=1.0.0; charset=utf-8",
        )
        .body(buffer);
    match res {
        Ok(r) => r.into_response(),
        Err(e) => {
            tracing::error!("{e}");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
