use axum::extract::State;
use reqwest::StatusCode;
use tracing::instrument;

use crate::types::*;

#[instrument]
pub async fn list_user_likes(
    State(AppState { db }): State<AppState>,
    ValidSession(user): ValidSession,
    UrlPath(theme): UrlPath<Ulid>,
) -> Result<(), StatusCode> {
    let ReadData { owner, flatten: _ } = db
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
