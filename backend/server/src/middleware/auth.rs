use crate::state::AppState;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::{TypedHeader, headers::UserAgent, extract::CookieJar};
use hyper::header::AUTHORIZATION;
use types::error::ApiError;

pub async fn auth(
    State(state): State<AppState>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, ApiError> {
    // Try to get token from Authorization header first
    let token = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| {
            if h.starts_with("Bearer ") {
                Some(h[7..].to_string())
            } else {
                None
            }
        });

    // If no Bearer token, try to get from SID cookie
    let session_token = match token {
        Some(token) => token,
        None => {
            match jar.get("SID") {
                Some(cookie) => cookie.value().to_string(),
                None => return Err(ApiError::SessionInvalid),
            }
        }
    };

    let session_token = match session_token.parse() {
        Ok(token) => token,
        Err(_) => return Err(ApiError::SessionInvalid),
    };

    let user = state
        .service
        .user
        .get_user_by_session_id(&session_token, user_agent.as_str())
        .await?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
