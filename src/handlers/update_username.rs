use axum::extract::State;
use reqwest::StatusCode;
use tracing::instrument;

use crate::{Session, types::*};

/// Returns 400 if new_username isn't unique
#[instrument]
pub async fn update_username(
    Session(user_ulid): Session,
    State(AppState { db }): State<AppState>,
    new_username: String,
) -> StatusCode {
    match db.update_username(&user_ulid, &new_username).await {
        Err(e) => match e {
            SqlxError::Database(e) => {
                if e.is_unique_violation() {
                    StatusCode::BAD_REQUEST
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        },
        Ok(_) => StatusCode::OK,
    }
}
