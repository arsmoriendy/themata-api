use std::sync::Arc;

use axum::extract::State;
use axum_valid::Valid;
use reqwest::StatusCode;
use tracing::instrument;

use crate::{CreateData, DB, Session, types::*};

#[instrument]
pub async fn create(
    Session(user_ulid): Session,
    State(db): State<Arc<DB>>,
    Valid(AxumJson(create_data)): Valid<AxumJson<CreateData>>,
) -> Result<String, StatusCode> {
    db.create_theme(&create_data, &user_ulid)
        .await
        .map(|u| u.0.to_string())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
