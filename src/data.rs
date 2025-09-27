use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{ColorSchemes, types::*};

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct CreateData {
    pub name: String,
    #[sqlx(json)]
    pub schemes: ColorSchemes,
    pub description: Option<String>,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct ReadData {
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub flatten: CreateData,
    pub owner: Ulid,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct ListData {
    pub ulid: Ulid,
    #[sqlx(flatten)]
    #[serde(flatten)]
    pub flatten: ReadData,
}

pub type UpdateData = CreateData;
