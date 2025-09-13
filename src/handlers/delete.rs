use std::sync::Arc;

use axum::extract::State;
use reqwest::StatusCode;

use crate::{DB, Session, types::*, ulid::Ulid};

pub async fn delete(
    Session(user_ulid): Session,
    State(db): State<Arc<DB>>,
    UrlPath(theme_ulid): UrlPath<Ulid>,
) -> StatusCode {
    let Ok(res) = db.read_theme_owner(&theme_ulid).await else {
        return StatusCode::INTERNAL_SERVER_ERROR;
    };

    let Some(theme_owner) = res else {
        return StatusCode::NOT_FOUND;
    };

    if theme_owner != user_ulid {
        return StatusCode::FORBIDDEN;
    }

    match db.delete_theme(&theme_ulid).await {
        Ok(()) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
