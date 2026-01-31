use axum::extract::State;
use reqwest::StatusCode;
use tracing::instrument;

use crate::{ReadData, types::*, ulid::Ulid};

#[instrument(skip(db))]
pub async fn read(
    State(AppState { db }): State<AppState>,
    UrlPath(ulid): UrlPath<Ulid>,
) -> Result<AxumJson<ReadData>, StatusCode> {
    let row = db
        .read_theme(&ulid)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(read_data) = row else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(AxumJson(read_data))
}
