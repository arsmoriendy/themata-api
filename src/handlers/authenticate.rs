use tracing::instrument;

use crate::Session;

#[instrument]
pub async fn authenticate(_: Session) {}
