use axum::extract::State;
use reqwest::StatusCode;
use tracing::instrument;

use crate::{types::*, ulid::Ulid};

#[instrument(skip(db, metrics))]
pub async fn delete(
    ValidSession(user_ulid): ValidSession,
    State(AppState { db, metrics }): State<AppState>,
    UrlPath(theme_ulid): UrlPath<Ulid>,
) -> StatusCode {
    let _latency_observer = metrics.observe_req_latency("delete");

    let Ok(res) = db.read_theme_owner(&theme_ulid).await else {
        return StatusCode::INTERNAL_SERVER_ERROR;
    };

    let Some(theme_owner) = res else {
        return StatusCode::NOT_FOUND;
    };

    if theme_owner != user_ulid {
        return StatusCode::FORBIDDEN;
    }

    match db.delete_theme(&theme_ulid).await {
        Ok(()) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
