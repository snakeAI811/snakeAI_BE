use crate::state::AppState;
use axum::{
    extract::{Query, State},
    Extension, Json,
};
use types::{
    dto::{GetRewardsQuery, SetWalletAddressRequest},
    error::{ApiError, ValidatedRequest},
    model::{Reward, User},
};

pub async fn get_me(Extension(user): Extension<User>) -> Result<Json<User>, ApiError> {
    Ok(Json(user))
}

pub async fn set_wallet_address(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    ValidatedRequest(payload): ValidatedRequest<SetWalletAddressRequest>,
) -> Result<Json<User>, ApiError> {
    if let Some(wallet_address) = user.wallet_address {
        if !wallet_address.is_empty() {
            return Err(ApiError::BadRequest(
                "Wallet address is already set".to_string(),
            ));
        }
    }

    // TODO: validate wallet address

    if state
        .service
        .user
        .get_user_by_wallet_address(&payload.wallet_address)
        .await?
        .is_some()
    {
        return Err(ApiError::BadRequest(
            "Wallet address is already using by other user".to_string(),
        ));
    }

    let user = state
        .service
        .user
        .set_wallet_address(&user.id, &payload.wallet_address)
        .await?;

    Ok(Json(user))
}

pub async fn get_rewards(
    Extension(user): Extension<User>,
    Query(opts): Query<GetRewardsQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Reward>>, ApiError> {
    let rewards = state
        .service
        .reward
        .get_rewards(&Some(user.id), opts.offset, opts.limit, opts.available)
        .await?;

    Ok(Json(rewards))
}
