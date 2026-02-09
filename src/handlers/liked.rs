use axum::extract::State;
use reqwest::StatusCode;
use tracing::instrument;

use crate::types::*;

#[instrument(skip(db, metrics))]
pub async fn liked(
    ValidSession(user): ValidSession,
    State(AppState { db, metrics }): State<AppState>,
    UrlPath(theme): UrlPath<Ulid>,
) -> Result<AxumJson<bool>, StatusCode> {
    let _latency_observer = metrics.observe_req_latency("liked");

    let liked = db
        .liked(&theme, &user)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(AxumJson(liked))
}
