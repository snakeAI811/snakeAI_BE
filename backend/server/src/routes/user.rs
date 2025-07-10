use crate::{
    handler::user::{
        get_claim_tx, get_me, get_mining_status, get_profile, get_rewards, get_tweets, set_wallet_address,
        token_validation, get_user_mining_status, get_user_profile, set_user_wallet_address,
        update_patron_status, update_user_role, update_lock_details, get_user_phase2_tweets,
    },
    state::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        // Existing authenticated routes
        .route("/token", get(token_validation))
        .route("/me", get(get_me))
        .route("/profile", get(get_profile))
        .route("/mining_status", get(get_mining_status))
        .route("/wallet_address", post(set_wallet_address))
        .route("/rewards", get(get_rewards))
        .route("/tweets", get(get_tweets))
        .route("/claim_tx", post(get_claim_tx))
        // New Patron Framework routes (by user ID)
        .route("/{user_id}", get(get_user_profile))
        .route("/{user_id}/mining_status", get(get_user_mining_status))
        .route("/{user_id}/wallet", post(set_user_wallet_address))
        .route("/{user_id}/patron_status", post(update_patron_status))
        .route("/{user_id}/role", post(update_user_role))
        .route("/{user_id}/lock_details", post(update_lock_details))
        .route("/{user_id}/phase2_tweets", get(get_user_phase2_tweets))
}
