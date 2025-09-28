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

pub type ColorSchemes = Vec<ColorSchemeEntry>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorSchemeEntry(String, ColorScheme);

pub type ColorScheme = Vec<RgbEntry>;

#[derive(Serialize, Deserialize, Debug)]
pub struct RgbEntry(String, Rgb);

#[derive(Serialize, Deserialize, Debug)]
pub struct Rgb(u8, u8, u8);
