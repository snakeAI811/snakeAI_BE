use crate::state::AppState;
use axum::{
    Extension, Json,
    extract::{State, Path},
};
use types::{
    error::ApiError,
    model::User,
};

/// Get swap statistics
pub async fn get_swap_stats(
    Extension(_user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<types::dto::SwapStatsResponse>, ApiError> {
    match state.service.otc_swap.get_swap_stats().await {
        Ok(stats) => Ok(Json(stats)),
        Err(err) => Err(ApiError::InternalServerError(err.to_string())),
    }
}

/// Get swap by PDA
pub async fn get_swap_by_pda(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path(pda): Path<String>,
) -> Result<Json<Option<types::dto::OtcSwapResponse>>, ApiError> {
    let user_role = user.role.as_deref().unwrap_or("none");
    
    match state.service.otc_swap.get_swap_by_pda(&pda, user_role).await {
        Ok(swap) => Ok(Json(swap)),
        Err(err) => Err(ApiError::InternalServerError(err.to_string())),
    }
}