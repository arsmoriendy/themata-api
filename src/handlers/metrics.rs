use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use prometheus_client::{encoding::text::encode, registry::Registry};
use reqwest::{StatusCode, header::CONTENT_TYPE};
use tracing::instrument;

#[instrument(skip(registry))]
pub async fn metrics(State(registry): State<Arc<Registry>>) -> Response {
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
