mod env;
mod handlers;
mod types;
mod ulid;

use axum::{
    Router,
    extract::FromRequestParts,
    http::request,
    routing::{delete, get, patch, post, put},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use dotenvy::dotenv;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Postgres, query, query_as, query_scalar};
use std::sync::Arc;
use ulid::{PrimitiveUlid, Ulid};

use crate::{env::ensure_envs, types::*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load .env
    dotenv()?;
    ensure_envs();

    // start logging
    tracing_subscriber::fmt::init();

    // init db
    sqlx::any::install_default_drivers();
    let db = DB {
        pool: Pool::connect_lazy(&env::DB_URL)?,
    };

    let app = Router::new()
        .route("/list", get(handlers::list))
        .route("/create/{name}", post(handlers::create))
        .route("/read/{ulid}", get(handlers::read))
        .route("/update/{ulid}", put(handlers::update))
        .route("/delete/{ulid}", delete(handlers::delete))
        .route("/login/github", get(handlers::github_login))
        .route("/update_username", patch(handlers::update_username))
        .route("/authenticate", get(handlers::authenticate))
        .with_state(Arc::new(db));

    let listener = tokio::net::TcpListener::bind(&*env::LISTEN_ADDR).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn get_session_user(jwt: &str) -> Result<Ulid, JwtError> {
    let header = jsonwebtoken::decode_header(jwt)?;

    let validator = {
        let mut validator = jsonwebtoken::Validation::new(header.alg);
        validator.set_required_spec_claims(&["sub"]);
        validator
    };

    let decoded = jsonwebtoken::decode::<JWTSessionClaims>(
        jwt,
        &jsonwebtoken::DecodingKey::from_secret(env::JWT_SECRET.as_ref()),
        &validator,
    )?;

    Ok(decoded.claims.sub)
}

/// Session JWT extractor + validator from authentication header
struct Session(Ulid);

impl FromRequestParts<Arc<DB>> for Session {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut request::Parts,
        db: &Arc<DB>,
    ) -> Result<Self, Self::Rejection> {
        // inherit bearer extractor
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, &())
                .await
                .map_err(|_| StatusCode::BAD_REQUEST)?;

        let session = bearer.token();

        let user_ulid = get_session_user(session).map_err(|e| match e.kind() {
            JwtErrorKind::InvalidSignature => StatusCode::FORBIDDEN,
            _ => StatusCode::BAD_REQUEST,
        })?;

        let user_exists = db
            .check_user_exists(&user_ulid)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        if !user_exists {
            return Err(StatusCode::FORBIDDEN);
        }

        Ok(Session(user_ulid))
    }
}

#[derive(Serialize, Deserialize)]
struct JWTSessionClaims {
    sub: Ulid,
}

struct DB {
    pool: Pool<Postgres>,
}

#[derive(FromRow, Serialize, Deserialize)]
struct ThemeWithName {
    name: String,
    #[sqlx(json)]
    schemes: Theme,
}

type Theme = Vec<ColorSchemeEntry>;

#[derive(Serialize, Deserialize)]
struct ColorSchemeEntry(String, ColorScheme);

type ColorScheme = Vec<RgbEntry>;

#[derive(Serialize, Deserialize)]
struct RgbEntry(String, Rgb);

#[derive(Serialize, Deserialize)]
struct Rgb(u8, u8, u8);

impl DB {
    async fn read_theme(&self, ulid: &Ulid) -> Result<Option<ThemeWithName>, SqlxError> {
        query_as("SELECT name, schemes FROM themes WHERE ulid = $1")
            .bind(ulid)
            .fetch_optional(&self.pool)
            .await
    }

    async fn read_theme_owner(&self, theme_ulid: &Ulid) -> Result<Option<Ulid>, SqlxError> {
        query_scalar("SELECT owner FROM themes WHERE ulid = $1")
            .bind(theme_ulid)
            .fetch_optional(&self.pool)
            .await
    }

    async fn create_theme(
        &self,
        name: &str,
        schemes: &SqlxJson<Theme>,
        owner: &Ulid,
    ) -> Result<Ulid, SqlxError> {
        query_scalar("INSERT INTO themes (ulid, name, schemes, owner) VALUES ($1, $2, $3, $4) RETURNING ulid")
            .bind(Ulid(PrimitiveUlid::new()))
            .bind(name)
            .bind(schemes)
            .bind(owner)
            .fetch_one(&self.pool)
            .await
    }

    async fn list_themes(&self, page: i64, per_page: i64) -> Result<Vec<Ulid>, SqlxError> {
        query_scalar("SELECT ulid FROM themes ORDER BY ulid LIMIT $1 OFFSET $2")
            .bind(per_page)
            .bind((page - 1) * per_page)
            .fetch_all(&self.pool)
            .await
    }

    async fn update_theme(
        &self,
        ulid: &Ulid,
        theme: &SqlxJson<ThemeWithName>,
    ) -> Result<(), SqlxError> {
        query("UPDATE themes SET name = $1, schemes = $2 WHERE ulid = $3")
            .bind(&theme.name)
            .bind(SqlxJson(&theme.schemes))
            .bind(ulid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_theme(&self, ulid: &Ulid) -> Result<(), SqlxError> {
        let _ = query("DELETE FROM themes WHERE ulid = $1")
            .bind(ulid)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn create_user(&self, email: &str) -> Result<Ulid, SqlxError> {
        query_scalar("INSERT INTO users (ulid, email, username) VALUES ($1, $2, $3) RETURNING ulid")
            .bind(Ulid(PrimitiveUlid::new()))
            .bind(email)
            .bind(PrimitiveUlid::new().to_string())
            .fetch_one(&self.pool)
            .await
    }

    async fn update_username(&self, user_ulid: &Ulid, new_username: &str) -> Result<(), SqlxError> {
        query("UPDATE users SET username = $2 WHERE ulid = $1")
            .bind(user_ulid)
            .bind(new_username)
            .execute(&self.pool)
            .await
            .map(|_| ())
    }

    async fn read_user(&self, email: &str) -> Result<Option<Ulid>, SqlxError> {
        query_scalar("SELECT ulid FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
    }

    async fn check_user_exists(&self, user_ulid: &Ulid) -> Result<bool, SqlxError> {
        query("SELECT NULL FROM users WHERE ulid = $1")
            .bind(user_ulid)
            .fetch_optional(&self.pool)
            .await
            .map(|row| row.is_some())
    }
}
