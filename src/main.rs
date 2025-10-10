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
use sqlx::Pool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

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
        pool: Pool::connect(&env::DB_URL).await?,
    };

    let app = Router::new()
        .route("/list", get(handlers::list))
        .route("/create", post(handlers::create))
        .route("/read/{ulid}", get(handlers::read))
        .route("/update/{ulid}", put(handlers::update))
        .route("/delete/{ulid}", delete(handlers::delete))
        .route("/login/github", get(handlers::github_login))
        .route("/update_username", patch(handlers::update_username))
        .route("/read_username/{user_ulid}", get(handlers::read_username))
        .route("/authenticate", get(handlers::authenticate))
        .route("/count", get(handlers::count))
        .layer(CorsLayer::permissive())
        .with_state(AppState { db: Arc::new(db) });

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

impl FromRequestParts<AppState> for Session {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut request::Parts,
        AppState { db }: &AppState,
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
