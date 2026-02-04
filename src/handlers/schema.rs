use schemars::Schema;

use crate::types::*;

#[instrument]
pub async fn schema() -> AxumJson<Schema> {
    AxumJson(schema_for!(Theme))
}
