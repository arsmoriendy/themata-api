use std::sync::Arc;

use axum::extract::State;
use reqwest::StatusCode;

use crate::{DB, ThemeContent, types::*, ulid::Ulid};

pub async fn read(
    State(db): State<Arc<DB>>,
    UrlPath(ulid): UrlPath<Ulid>,
) -> Result<AxumJson<ThemeContent>, StatusCode> {
    let row = db
        .read_theme(&ulid)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(tc) = row else {
        return Err(StatusCode::NOT_FOUND);
    };

    Ok(AxumJson(tc))
}
