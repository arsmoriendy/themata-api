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
) -> Result<String, (StatusCode, &'static str)> {
    db.create_theme(&create_data, &user_ulid)
        .await
        .map(|u| u.0.to_string())
        .map_err(|e| {
            if let Some(e) = e.into_database_error() {
                let Some(e) = e.constraint() else {
                    return (StatusCode::INTERNAL_SERVER_ERROR, "database error");
                };
                if e == "max_themes" {
                    return (StatusCode::CONFLICT, "user exceeded maximum theme quota");
                }
            }
            (StatusCode::INTERNAL_SERVER_ERROR, "database protocol error")
        })
}
