pub mod data;
pub mod db;
pub mod external_exports;
pub mod internal_exports;

use std::sync::Arc;

pub use data::*;
pub use db::*;
pub use external_exports::*;
pub use internal_exports::*;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
}

#[derive(Serialize, Deserialize)]
pub struct JWTSessionClaims {
    pub sub: Ulid,
    pub email: String,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Validate, JsonSchema)]
pub struct Theme {
    #[validate(length(min = 1, max = 32))]
    pub name: String,
    #[sqlx(json)]
    #[validate(length(min = 1, max = 32), nested)]
    pub schemes: Vec<ColorScheme>,
    #[validate(length(max = 512))]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate, JsonSchema)]
pub struct ColorScheme {
    #[validate(length(min = 1, max = 32))]
    name: String,
    #[validate(length(min = 1, max = 32), nested)]
    colors: Vec<Color>,
}

#[derive(Serialize, Deserialize, Debug, Validate, JsonSchema)]
pub struct Color {
    #[validate(length(min = 1, max = 32))]
    name: String,
    rgb: Rgb,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct Rgb(u8, u8, u8);
