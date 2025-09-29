mod data;
mod db;
mod reexports;

pub use data::*;
pub use db::*;
pub use reexports::*;

#[derive(Serialize, Deserialize)]
pub struct JWTSessionClaims {
    pub sub: Ulid,
    pub preferred_username: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct ColorScheme {
    #[validate(length(min = 1, max = 32))]
    name: String,
    #[validate(length(min = 1, max = 32), nested)]
    colors: Vec<Color>,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Color {
    #[validate(length(min = 1, max = 32))]
    name: String,
    rgb: Rgb,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rgb(u8, u8, u8);
