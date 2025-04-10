use crate::state::AppState;
use axum::{extract::State, Extension, Json};
use types::{error::ApiError, model::User};

pub async fn get_me(
    State(state): State<AppState>,
    Extension(me): Extension<User>,
) -> Result<Json<User>, ApiError> {
    Ok(Json(me))
}
