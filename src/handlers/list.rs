use axum::extract::State;
use reqwest::StatusCode;
use tracing::instrument;

use crate::{ListData, types::*};

#[instrument]
pub async fn list(
    State(AppState { db }): State<AppState>,
    QueryMap(qm): QueryMap,
) -> Result<AxumJson<Vec<ListData>>, StatusCode> {
    const MAX_PER_PAGE: i64 = 100;
    const DEFAULT_PER_PAGE: i64 = 10;

    let mut page = 1;
    let mut per_page = DEFAULT_PER_PAGE;
    let mut filters: Vec<ListFilter> = vec![];

    for (k, v) in &qm {
        match k.as_str() {
            "page" => page = v.parse::<i64>().map_err(|_| StatusCode::BAD_REQUEST)?,
            "per_page" => {
                per_page = v.parse::<i64>().map_err(|_| StatusCode::BAD_REQUEST)?;
                if per_page > MAX_PER_PAGE {
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
            "search" => filters.push(ListFilter::SEARCH(v.as_str())),
            "owner" => filters.push(ListFilter::OWNER(Ulid(
                PrimitiveUlid::try_from(v.as_str()).map_err(|_| StatusCode::BAD_REQUEST)?,
            ))),
            _ => {}
        };
    }

    db.list_themes(page, per_page, &filters)
        .await
        .map(AxumJson)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
