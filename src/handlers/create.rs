use std::sync::Arc;

use axum::extract::State;
use reqwest::StatusCode;

use crate::{DB, Schemes, Session, types::*};

pub async fn create(
    Session(user_ulid): Session,
    State(db): State<Arc<DB>>,
    UrlPath(name): UrlPath<String>,
    AxumJson(schemes): AxumJson<Schemes>,
) -> Result<String, StatusCode> {
    db.create_theme(&name, &SqlxJson(schemes), &user_ulid)
        .await
        .map(|u| u.0.to_string())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
