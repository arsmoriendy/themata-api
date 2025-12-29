use axum::extract::State;
use reqwest::StatusCode;
use tracing::instrument;

use crate::types::*;

#[instrument]
pub async fn unlike(
    State(AppState { db }): State<AppState>,
    Session(user): Session,
    UrlPath(theme): UrlPath<Ulid>,
) -> Result<(), StatusCode> {
    db.unlike(&theme, &user)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(())
}
