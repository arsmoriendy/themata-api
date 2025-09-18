use std::sync::Arc;

use axum::extract::State;
use reqwest::StatusCode;
use tracing::instrument;

use crate::{DB, ReadData, types::*, ulid::Ulid};

#[instrument]
pub async fn read(
    State(db): State<Arc<DB>>,
    UrlPath(ulid): UrlPath<Ulid>,
) -> Result<AxumJson<ReadData>, StatusCode> {
    let row = db
        .read_theme(&ulid)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(tc) = row else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(AxumJson(tc))
}
