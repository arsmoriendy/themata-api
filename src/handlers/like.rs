use axum::extract::State;
use reqwest::StatusCode;
use tracing::instrument;

use crate::types::*;

#[instrument(skip(db, metrics))]
pub async fn like(
    State(AppState { db, metrics }): State<AppState>,
    ValidSession(user): ValidSession,
    UrlPath(theme): UrlPath<Ulid>,
) -> Result<(), StatusCode> {
    let _latency_observer = metrics.observe_req_latency("like");

    let ReadData { owner, .. } = db
        .read_theme(&theme)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .ok_or(StatusCode::BAD_REQUEST)?;
    if owner == user {
        return Err(StatusCode::BAD_REQUEST);
    }
    db.like(&theme, &user)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(())
}
