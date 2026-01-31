use axum::extract::State;
use axum_valid::Valid;
use reqwest::StatusCode;
use tracing::instrument;

use crate::{CreateData, types::*};

#[instrument(skip(db))]
pub async fn create(
    ValidSession(user_ulid): ValidSession,
    State(AppState { db }): State<AppState>,
    Valid(AxumJson(create_data)): Valid<AxumJson<CreateData>>,
) -> Result<String, StatusCode> {
    db.create_theme(&create_data, &user_ulid)
        .await
        .map(|u| u.0.to_string())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
