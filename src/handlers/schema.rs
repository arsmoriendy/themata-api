use axum::extract::State;
use schemars::Schema;

use crate::types::*;

#[instrument(skip(metrics))]
pub async fn schema(State(AppState { db: _, metrics }): State<AppState>) -> AxumJson<Schema> {
    let _latency_observer = metrics.observe_req_latency("schema");

    AxumJson(schema_for!(Theme))
}
