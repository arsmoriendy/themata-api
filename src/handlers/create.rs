use axum::extract::State;
use axum_valid::Valid;
use reqwest::StatusCode;
use tracing::instrument;

use crate::{CreateData, types::*};

#[instrument(skip(db, metrics))]
pub async fn create(
    ValidSession(user_ulid): ValidSession,
    State(AppState { db, metrics }): State<AppState>,
    Valid(AxumJson(create_data)): Valid<AxumJson<CreateData>>,
) -> Response {
    let _latency_observer = metrics.observe_req_latency("create");

    match db.create_theme(&create_data, &user_ulid).await {
        Ok(ulid) => ulid.0.to_string().into_response(),
        Err(e) => {
            if let Some(e) = e.as_database_error()
                && let Some(e) = e.constraint()
                && e == "max_themes"
            {
                (StatusCode::CONFLICT, "user exceeded maximum theme quota").into_response()
            } else {
                tracing::error!("{e}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
