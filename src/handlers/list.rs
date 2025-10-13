use axum::extract::State;
use reqwest::StatusCode;
use serde::Deserialize;
use tracing::instrument;

use crate::{ListData, types::*};

#[derive(Deserialize, Debug)]
pub struct UrlParam {
    page: Option<i64>,
    per_page: Option<i64>,
    search: Option<String>,
}

#[instrument]
pub async fn list(
    State(AppState { db }): State<AppState>,
    UrlQuery(UrlParam {
        page,
        per_page,
        search,
    }): UrlQuery<UrlParam>,
) -> Result<AxumJson<Vec<ListData>>, StatusCode> {
    const MAX_PER_PAGE: i64 = 100;
    const DEFAULT_PER_PAGE: i64 = 10;

    if let Some(per_page) = per_page
        && per_page > MAX_PER_PAGE
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    db.list_themes(
        page.unwrap_or(1),
        per_page.unwrap_or(DEFAULT_PER_PAGE),
        search.as_ref().map(|s| s.as_ref()),
    )
    .await
    .map(AxumJson)
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
