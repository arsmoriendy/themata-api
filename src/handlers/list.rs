use axum::extract::State;
use reqwest::StatusCode;
use serde::Deserialize;
use tracing::instrument;

use crate::{ListData, types::*};

#[derive(Deserialize, Debug)]
pub struct Pagination {
    page: Option<i64>,
    per_page: Option<i64>,
}

#[instrument]
pub async fn list(
    State(AppState { db }): State<AppState>,
    UrlQuery(pagination): UrlQuery<Pagination>,
) -> Result<AxumJson<Vec<ListData>>, StatusCode> {
    const MAX_PER_PAGE: i64 = 100;
    const DEFAULT_PER_PAGE: i64 = 10;

    if let Some(per_page) = pagination.per_page
        && per_page > MAX_PER_PAGE
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    db.list_themes(
        pagination.page.unwrap_or(1),
        pagination.per_page.unwrap_or(DEFAULT_PER_PAGE),
    )
    .await
    .map(AxumJson)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
