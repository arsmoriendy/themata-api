use axum::extract::State;
use axum_valid::Valid;
use reqwest::StatusCode;
use tracing::instrument;

use crate::{UpdateData, types::*, ulid::Ulid};

#[instrument(skip(db, metrics))]
pub async fn update(
    ValidSession(user_ulid): ValidSession,
    State(AppState { db, metrics }): State<AppState>,
    UrlPath(theme_ulid): UrlPath<Ulid>,
    Valid(AxumJson(update_data)): Valid<AxumJson<UpdateData>>,
) -> StatusCode {
    let _latency_observer = metrics.observe_req_latency("update");

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
