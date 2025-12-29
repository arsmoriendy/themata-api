mod env;
mod extractors;
mod handlers;
mod types;
mod ulid;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use dotenvy::dotenv;
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
        .route("/authenticate", get(handlers::authenticate))
        .route("/count", get(handlers::count))
        .route("/like/{ulid}", post(handlers::like))
        .route("/unlike/{ulid}", delete(handlers::unlike))
        .route("/liked/{ulid}", get(handlers::liked))
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
