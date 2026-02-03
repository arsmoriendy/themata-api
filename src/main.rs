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
    let db = Arc::new(DB {
        pool: Pool::connect(&env::DATABASE_URL).await?,
    });

    // init redis
    let rd = redis::Client::open(env::REDIS_URL.clone())?
        .get_multiplexed_async_connection()
        .await?;

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
        .with_state(AppState { db: db.clone() });

    let listener = tokio::net::TcpListener::bind(&*env::LISTEN_ADDR).await?;
    let server_handle = tokio::spawn(async { axum::serve(listener, app).await });

    let rd2 = rd.clone();
    let db2 = db.clone();
    let agg_views_handle = tokio::spawn(async { agg_views(rd2, db2).await });

    server_handle.await??;
    agg_views_handle.await??;
    Ok(())
}

const REDIS_VIEW_KEYSET_KEY: &str = "keys:views";
const AGGREGATE_VIEWS_INTERVAL: Duration = Duration::from_secs(1);

#[tracing::instrument(skip_all)]
async fn agg_views(
    mut rd: redis::aio::MultiplexedConnection,
    db: Arc<DB>,
) -> Result<(), redis::RedisError> {
    use redis::AsyncTypedCommands;

    let mut ticker = tokio::time::interval(AGGREGATE_VIEWS_INTERVAL);
    loop {
        ticker.tick().await;

        let views_keyset = rd.smembers(REDIS_VIEW_KEYSET_KEY).await?;
        for view_key in views_keyset {
            // retreive ulid from key
            let ulid_str = &view_key[6..];
            let Ok(primitive_ulid) = PrimitiveUlid::from_string(&ulid_str) else {
                tracing::warn!("malformed redis view key: {}", &view_key);
                continue;
            };
            let ulid = Ulid(primitive_ulid);

            // convert vews
            let Ok(i64_views) = i64::try_from(rd.scard(&view_key).await?) else {
                tracing::error!("failed to convert views to i64");
                continue;
            };

            // incement db
            let Ok(()) = db.increment_views_by(&ulid, i64_views).await else {
                tracing::warn!(
                    "redis view key ulid: {}, does not exist in database",
                    ulid.0
                );
                continue;
            };

            rd.del(&view_key).await?;
        }

        rd.del(REDIS_VIEW_KEYSET_KEY).await?;
    }
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
