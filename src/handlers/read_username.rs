use std::sync::Arc;

use axum::extract::State;
use reqwest::StatusCode;

use crate::{DB, types::*};

/// Returns 400 if new_username isn't unique
pub async fn read_username(
    UrlPath(user_ulid): UrlPath<Ulid>,
    State(db): State<Arc<DB>>,
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
