use axum::extract::State;
use tracing::instrument;

use crate::{Session, types::AppState};

#[instrument(skip(metrics))]
pub async fn authenticate(State(AppState { db: _, metrics }): State<AppState>, _: Session) {
    let _latency_observer = metrics.observe_req_latency("authenticate");
}
