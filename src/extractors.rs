use std::collections::HashMap;

use crate::{
    get_session_user,
    types::{AppState, external_exports::*},
    ulid::Ulid,
};

/// Session JWT extractor + validator from authentication header
pub struct Session(pub Ulid);

impl FromRequestParts<AppState> for Session {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut RequestParts,
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

pub struct QueryMap(pub HashMap<String, String>);

impl<S: Sync> FromRequestParts<S> for QueryMap {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        let Some(query) = parts.uri.query() else {
            return Ok(QueryMap(HashMap::new()));
        };

        serde_urlencoded::from_str(query)
            .map(|qm| QueryMap(qm))
            .map_err(|_| StatusCode::BAD_REQUEST)
    }
}
