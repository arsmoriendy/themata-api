use axum::extract::State;
use reqwest::StatusCode;
use tracing::instrument;

use crate::types::*;

#[instrument(skip(db))]
pub async fn like(
    State(AppState { db }): State<AppState>,
    ValidSession(user): ValidSession,
    UrlPath(theme): UrlPath<Ulid>,
) -> Result<(), StatusCode> {
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
