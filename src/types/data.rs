use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::types::*;

#[derive(FromRow, Serialize, Deserialize, Debug, Validate)]
pub struct CreateData {
    #[validate(length(min = 1, max = 32))]
    pub name: String,
    #[sqlx(json)]
    #[validate(length(min = 1, max = 32), nested)]
    pub schemes: Vec<ColorScheme>,
    #[validate(length(min = 1, max = 512))]
    pub description: Option<String>,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Validate)]
pub struct ReadData {
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub flatten: CreateData,
    pub owner: Ulid,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Validate)]
pub struct ListData {
    pub ulid: Ulid,
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub flatten: ReadData,
}

pub type UpdateData = CreateData;
