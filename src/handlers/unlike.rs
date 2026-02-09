use axum::extract::State;
use reqwest::StatusCode;
use tracing::instrument;

use crate::types::*;

#[instrument(skip(db, metrics))]
pub async fn unlike(
    State(AppState { db, metrics }): State<AppState>,
    ValidSession(user): ValidSession,
    UrlPath(theme): UrlPath<Ulid>,
) -> Result<(), StatusCode> {
    let _latency_observer = metrics.observe_req_latency("unlike");

    db.unlike(&theme, &user)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(())
}
