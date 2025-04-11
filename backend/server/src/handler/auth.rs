use crate::state::{AppState, TwitterChallenge};
use axum::{
    extract::{ConnectInfo, Query, State},
    response::{IntoResponse, Redirect},
};
use axum_extra::{
    extract::{cookie::Cookie, CookieJar},
    headers::UserAgent,
    TypedHeader,
};
use serde::Deserialize;
use std::net::SocketAddr;
use twitter_v2::{
    authorization::Scope,
    oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier},
    TwitterApi,
};
use types::error::ApiError;

pub async fn login(State(state): State<AppState>) -> impl IntoResponse {
    let mut ctx = state.ctx.lock().unwrap();
    ctx.remove_expired_challenges();
    // create challenge
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
    // create authorization url
    let (url, state) = ctx.client.auth_url(
        challenge,
        [
            Scope::TweetRead,
            Scope::TweetWrite,
            Scope::UsersRead,
            Scope::OfflineAccess,
        ],
    );
    dbg!(url.to_string());

    ctx.challenges
        .insert(state.secret().clone(), TwitterChallenge::new(verifier));
    Redirect::to(&url.to_string())
}

#[derive(Deserialize)]
pub struct CallbackParams {
    code: AuthorizationCode,
    state: CsrfToken,
}

pub async fn callback(
    State(s): State<AppState>,
    Query(CallbackParams { code, state }): Query<CallbackParams>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    jar: CookieJar,
) -> Result<impl IntoResponse, ApiError> {
    let (client, verifier) = {
        let ctx = s.ctx.lock().unwrap();

        if let Some(challenge) = ctx.challenges.get(state.secret()) {
            let client = ctx.client.clone();
            (
                client,
                PkceCodeVerifier::new(challenge.verifier.secret().clone()),
            )
        } else {
            return Err(ApiError::BadRequest("Invalid state returned".to_string()));
        }
    };

    let token = client
        .request_token(code, PkceCodeVerifier::from(verifier))
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    let api = TwitterApi::new(token);
    let user_response = api
        .get_users_me()
        .send()
        .await
        .map_err(|err| ApiError::InternalServerError(err.to_string()))?;

    let twitter_user = match &user_response.data {
        Some(me) => me,
        None => {
            return Err(ApiError::InternalServerError(
                "An error occurred while trying to retrieve user information from twitter"
                    .to_string(),
            ));
        }
    };

    let user = s
        .service
        .user
        .insert_user(&twitter_user.id.to_string(), &twitter_user.username)
        .await?;

    let session = s
        .service
        .session
        .create_session(
            &user.id,
            user_agent.as_str(),
            &addr.ip().to_string(),
            s.env.session_ttl_in_minutes,
        )
        .await?;

    // Create a cookie for the session ID
    let cookie = Cookie::build(("SID", session.session_id.to_string()))
        .path("/")
        .http_only(true)
        .secure(true) // Use only if served over HTTPS
        .same_site(axum_extra::extract::cookie::SameSite::Lax);

    // Attach the cookie to the response
    Ok((
        jar.add(cookie),
        Redirect::to(&format!(
            "{}?SID={}",
            s.env.frontend_url, session.session_id
        )),
    ))
}
