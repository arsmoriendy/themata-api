use axum::extract::State;
use reqwest::StatusCode;
use tracing::instrument;

use crate::types::*;

/// Returns 400 if new_username isn't unique
#[instrument]
pub async fn read_username(
    UrlPath(user_ulid): UrlPath<Ulid>,
    State(AppState { db }): State<AppState>,
) -> Result<String, StatusCode> {
    let Some(username) = db
        .read_username(&user_ulid)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(username)
}
