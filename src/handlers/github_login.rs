use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use reqwest::{ClientBuilder, StatusCode, header::ACCEPT};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{DB, JWTSessionClaims, env, types::*};

#[derive(Deserialize, Debug)]
pub struct GithubQueryParams {
    code: String,
}

#[instrument]
pub async fn github_login(
    State(db): State<Arc<DB>>,
    UrlQuery(GithubQueryParams { code }): UrlQuery<GithubQueryParams>,
) -> Response {
    // TODO: handle errors better
    let client = match ClientBuilder::new()
        .user_agent(&*env::GITHUB_APP_NAME)
        .build()
    {
        Ok(client) => client,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let access_token = {
        #[derive(Deserialize, Serialize)]
        struct ErrorResponse {
            error: String,
            error_description: String,
            error_uri: String,
        }

        #[derive(Deserialize, Serialize)]
        struct SuccessResponse {
            access_token: String,
        }

        #[derive(Deserialize, Serialize)]
        struct JsonResponse {
            #[serde(skip_serializing_if = "Option::is_none", flatten)]
            error: Option<ErrorResponse>,
            #[serde(skip_serializing_if = "Option::is_none", flatten)]
            success: Option<SuccessResponse>,
        }

        let res = match client
            .post("https://github.com/login/oauth/access_token")
            .query(&[("client_id", &*env::GITHUB_CLIENT_ID)])
            .query(&[("client_secret", &*env::GITHUB_CLIENT_SECRET)])
            .query(&[("code", code)])
            .header(ACCEPT, "application/json")
            .send()
            .await
        {
            Ok(res) => res,
            Err(_) => return StatusCode::BAD_GATEWAY.into_response(),
        };

        // parse json
        let json = match res.json::<JsonResponse>().await {
            Ok(json) => json,
            Err(_) => return StatusCode::BAD_GATEWAY.into_response(),
        };

        // handle response errors, because github always gives status code 200
        if let Some(e) = json.error {
            return (StatusCode::BAD_REQUEST, AxumJson(e)).into_response();
        }

        match json.success {
            // handle if github gives neither error or success
            None => return StatusCode::BAD_GATEWAY.into_response(),
            Some(success) => success.access_token,
        }
    };

    let emails = {
        let res = match client
            .get("https://api.github.com/user/emails")
            .header(ACCEPT, "application/json")
            .bearer_auth(access_token)
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
        {
            Ok(res) => res,
            Err(_) => return StatusCode::BAD_GATEWAY.into_response(),
        };

        #[derive(Deserialize)]
        struct Email {
            email: String,
            verified: bool,
            primary: bool,
        }

        match res.json::<Vec<Email>>().await {
            Ok(emails) => emails,
            Err(_) => return StatusCode::BAD_GATEWAY.into_response(),
        }
    };

    let primary_verified_email = match emails
        .iter()
        .find(|e| e.verified && e.primary)
        .map(|e| &e.email)
    {
        Some(e) => e,
        None => return StatusCode::NOT_FOUND.into_response(),
    };

    let user_ulid = {
        let Ok(row) = db.read_user(primary_verified_email).await else {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };

        match row {
            Some(ulid) => ulid,
            None => match db.create_user(primary_verified_email).await {
                Ok(ulid) => ulid,
                Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            },
        }
    };

    let Ok(Some(username)) = db.read_username(&user_ulid).await else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    let claim = JWTSessionClaims {
        sub: user_ulid,
        preferred_username: username,
    };

    let jwt = match jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &jsonwebtoken::EncodingKey::from_secret(env::JWT_SECRET.as_ref()),
    ) {
        Ok(jwt) => jwt,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    jwt.into_response()
}
