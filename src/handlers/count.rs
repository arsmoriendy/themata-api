use axum::extract::State;
use reqwest::StatusCode;

use crate::types::*;

pub async fn count(
    State(AppState { db, metrics }): State<AppState>,
) -> Result<AxumJson<u64>, StatusCode> {
    let _latency_observer = metrics.observe_req_latency("count");

    let Ok(count) = db.read_theme_count().await else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let Ok(count) = u64::try_from(count) else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    return Ok(AxumJson(count));
}
