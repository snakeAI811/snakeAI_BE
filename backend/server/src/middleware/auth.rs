use crate::state::AppState;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use hyper::header::AUTHORIZATION;
use types::error::ApiError;

pub async fn auth(
    State(state): State<AppState>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, ApiError> {
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

    let session_token = match token {
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
