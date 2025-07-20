use crate::state::AppState;
use axum::{Json, extract::State};
use axum_extra::{TypedHeader, headers::UserAgent, extract::{CookieJar, cookie::Cookie}};
use types::error::ApiError;
use serde_json::json;

/// Development-only endpoint to create a mock user session
/// This bypasses Twitter OAuth for local testing
pub async fn dev_login(
    State(state): State<AppState>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    jar: CookieJar,
) -> Result<impl axum::response::IntoResponse, ApiError> {
    // Only allow in development mode
    if state.env.production {
        return Err(ApiError::NotFound("Endpoint not available in production".to_string()));
    }

    // Create or get a test user
    let test_user = state
        .service
        .user
        .insert_user("test_twitter_id", "test_user")
        .await?;

    // Create a session for the test user
    let session = state
        .service
        .session
        .create_session(
            &test_user.id,
            user_agent.as_str(),
            "127.0.0.1",
            state.env.session_ttl_in_minutes,
        )
        .await?;

    // Create a cookie for the session ID
    let cookie = Cookie::build(("SID", session.session_id.to_string()))
        .path("/")
        .http_only(false) // Allow JS access for development
        .secure(false) // Allow HTTP for localhost
        .same_site(axum_extra::extract::cookie::SameSite::Lax);

    // Return session info as JSON for frontend
    Ok((
        jar.add(cookie),
        Json(json!({
            "success": true,
            "session_id": session.session_id.to_string(),
            "user": {
                "id": test_user.id,
                "twitter_username": test_user.twitter_username,
                "wallet_address": test_user.wallet_address
            }
        }))
    ))
}

/// Development endpoint to check current session
pub async fn dev_session_info(
    State(state): State<AppState>,
    jar: CookieJar,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if state.env.production {
        return Err(ApiError::NotFound("Endpoint not available in production".to_string()));
    }

    let session_id = jar.get("SID")
        .map(|cookie| cookie.value())
        .ok_or(ApiError::SessionInvalid)?;

    let session_uuid = session_id.parse()
        .map_err(|_| ApiError::SessionInvalid)?;

    let user = state
        .service
        .user
        .get_user_by_session_id(&session_uuid, user_agent.as_str())
        .await?;

    Ok(Json(json!({
        "authenticated": true,
        "session_id": session_id,
        "user": {
            "id": user.id,
            "twitter_username": user.twitter_username,
            "wallet_address": user.wallet_address
        }
    })))
}
