use axum::extract::State;
use reqwest::StatusCode;
use tracing::instrument;

use crate::types::*;

#[instrument]
pub async fn liked(
    ValidSession(user): ValidSession,
    State(AppState { db }): State<AppState>,
    UrlPath(theme): UrlPath<Ulid>,
) -> Result<AxumJson<bool>, StatusCode> {
    let liked = db
        .liked(&theme, &user)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(AxumJson(liked))
}
