use axum::extract::State;
use reqwest::StatusCode;
use tracing::instrument;

use crate::types::*;

#[instrument(skip(db))]
pub async fn unlike(
    State(AppState { db }): State<AppState>,
    ValidSession(user): ValidSession,
    UrlPath(theme): UrlPath<Ulid>,
) -> Result<(), StatusCode> {
    db.unlike(&theme, &user)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(())
}
