use serde::{Deserialize, Serialize};
use sqlx::types::chrono::*;

use crate::types::*;

pub type CreateData = Theme;

#[derive(FromRow, Serialize, Deserialize, Debug, Validate)]
pub struct ReadData {
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub flatten: CreateData,
    pub owner: Ulid,
    pub like_count: i64,
    pub views: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Validate)]
pub struct ListData {
    pub ulid: Ulid,
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub flatten: ReadData,
}

pub type UpdateData = CreateData;
