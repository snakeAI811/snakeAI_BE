use crate::state::AppState;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::{extract::cookie, headers::UserAgent, TypedHeader};
use types::error::ApiError;

pub async fn auth(
    State(state): State<AppState>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, ApiError> {
    let cookies = req
        .headers()
        .get("Cookie")
        .and_then(|h| h.to_str().ok())
        .ok_or(ApiError::SessionInvalid)?;

    let session_token = cookie::Cookie::split_parse(cookies)
        .filter_map(|c| c.ok())
        .find(|c| c.name() == "SID")
        .map(|c| c.value().to_string());

    let session_token = match session_token {
        Some(token) => token,
        None => return Err(ApiError::SessionInvalid),
    };

    let user = state
        .service
        .user
        .get_user_by_session_id(&session_token.parse().unwrap(), user_agent.as_str())
        .await?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
