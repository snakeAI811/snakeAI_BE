use crate::{
    handler::{
        patron_minimal::get_initialize_user_claim_tx,
        user::{
            get_claim_tx, get_me, get_mining_status, get_profile, get_rewards, get_tweets, set_wallet_address,
            token_validation, get_user_mining_status, get_user_profile, set_user_wallet_address,
            update_patron_status, update_user_role, update_lock_details, get_user_phase2_tweets,
            // Tweet mining endpoints
            get_tweet_mining_status, claim_tweet_reward_tx, set_reward_flag,
            // Data endpoints
            get_token_info, get_patron_application_status, get_active_swaps, get_my_swaps, get_vesting_info,
            // DAO endpoints
            get_dao_users, get_dao_user_count,
            // Smart contract interaction endpoints
            save_role_selection, select_role_tx, apply_patron_tx, approve_patron_tx, claim_tokens_with_role_tx,
            lock_tokens_tx, unlock_tokens_tx, claim_yield_tx, create_vesting_tx, withdraw_vesting_tx,
            initiate_otc_swap_tx, accept_otc_swap_tx, cancel_otc_swap_tx, update_otc_swap_tx_signature, debug_user_swaps, force_cancel_user_swap, check_patron_eligibility,
        },
        otc_swap::{get_swap_stats, get_swap_by_pda},
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
        .route("/total_mining_status", get(get_mining_status))
        .route("/wallet_address", post(set_wallet_address))
        .route("/rewards", get(get_rewards))
        .route("/tweets", get(get_tweets))
        .route("/claim_tx", post(get_claim_tx))
        .route("/set_reward_flag", post(set_reward_flag))
        // Tweet mining endpoints
        .route("/tweet_mining_status", get(get_tweet_mining_status))
        .route("/claim_tweet_reward", post(claim_tweet_reward_tx))
        // Data endpoints (replacing test endpoints)
        .route("/token_info", get(get_token_info))
        .route("/patron_application", get(get_patron_application_status))
        .route("/active_swaps", get(get_active_swaps))
        .route("/my_swaps", get(get_my_swaps))
        .route("/vesting_info", get(get_vesting_info))
        // DAO endpoints
        .route("/dao_users", get(get_dao_users))
        .route("/dao_user_count", get(get_dao_user_count))
        // Smart contract interaction routes
        .route("/initialize_user_claim", post(get_initialize_user_claim_tx))
        .route("/select_role", post(select_role_tx))
        .route("/save_role_selection", post(save_role_selection))
        .route("/check_patron_eligibility", post(check_patron_eligibility))
        .route("/apply_patron", post(apply_patron_tx))
        .route("/approve_patron", post(approve_patron_tx))
        .route("/claim_tokens_with_role", post(claim_tokens_with_role_tx))
        .route("/lock_tokens", post(lock_tokens_tx))
        .route("/unlock_tokens", post(unlock_tokens_tx))
        .route("/claim_yield", post(claim_yield_tx))
        .route("/create_vesting", post(create_vesting_tx))
        .route("/withdraw_vesting", post(withdraw_vesting_tx))
        // OTC swap endpoints
        .route("/initiate_otc_swap", post(initiate_otc_swap_tx))
        .route("/accept_otc_swap", post(accept_otc_swap_tx))
        .route("/cancel_otc_swap", post(cancel_otc_swap_tx))
        .route("/update_otc_swap_signature", post(update_otc_swap_tx_signature))
        .route("/debug_swaps", get(debug_user_swaps))
        .route("/force_cancel_swap", post(force_cancel_user_swap))
        .route("/swap_stats", get(get_swap_stats))
        .route("/swap/{pda}", get(get_swap_by_pda))
        // User ID specific routes (must be at the end to avoid conflicts)
        .route("/{user_id}", get(get_user_profile))
        .route("/{user_id}/mining_status", get(get_user_mining_status))
        .route("/{user_id}/wallet", post(set_user_wallet_address))
        .route("/{user_id}/patron_status", post(update_patron_status))
        .route("/{user_id}/role", post(update_user_role))
        .route("/{user_id}/lock_details", post(update_lock_details))
        .route("/{user_id}/phase2_tweets", get(get_user_phase2_tweets))
}
