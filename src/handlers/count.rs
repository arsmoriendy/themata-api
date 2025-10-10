use axum::extract::State;
use reqwest::StatusCode;

use crate::types::*;

pub async fn count(State(AppState { db }): State<AppState>) -> Result<AxumJson<u64>, StatusCode> {
    let Ok(count) = db.read_theme_count().await else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let Ok(count) = u64::try_from(count) else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    return Ok(AxumJson(count));
}
