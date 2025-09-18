use std::sync::Arc;

use axum::extract::State;
use reqwest::StatusCode;
use tracing::instrument;

use crate::{DB, Session, UpdateData, types::*, ulid::Ulid};

#[instrument]
pub async fn update(
    Session(user_ulid): Session,
    State(db): State<Arc<DB>>,
    UrlPath(theme_ulid): UrlPath<Ulid>,
    AxumJson(update_data): AxumJson<UpdateData>,
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

    if let Err(e) = db.update_theme(&theme_ulid, &SqlxJson(update_data)).await {
        return match e {
            SqlxError::RowNotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
    }

    StatusCode::OK
}
