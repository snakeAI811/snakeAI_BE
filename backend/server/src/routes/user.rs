use crate::{
    handler::{
        otc_swap::{
            accept_otc_swap_tx, 
            cancel_otc_swap_tx, 
            get_active_swaps, 
            get_my_swaps,
            get_swap_by_pda, 
            get_swap_stats, 
            initiate_otc_swap_enhanced_tx, 
            initiate_otc_swap_tx,
            process_cancel_otc_swap_tx, 
            update_otc_swap_tx_signature,
        },
        patron_minimal::get_initialize_user_claim_tx,
        tweet_template::{
            get_tweet_templates,
            post_tweet,
            get_fresh_templates,
        },
        user::{
            apply_patron_tx,
            approve_patron_tx,
            check_patron_eligibility,
            claim_tokens_with_role_tx,
            claim_tweet_reward_tx,
            claim_yield_tx,
            create_vesting_tx,
            batch_claim_tx,
            get_dao_user_count,
            // DAO endpoints
            get_dao_users,
            get_me,
            get_mining_status,
            get_patron_application_status,
            get_profile,
            get_rewards,
            // Data endpoints
            get_token_info,
            // Tweet mining endpoints
            get_tweet_mining_status,
            get_tweets,
            get_user_mining_status,
            get_user_phase2_tweets,
            get_user_profile,
            get_vesting_info,
            lock_tokens_tx,
            // Smart contract interaction endpoints
            save_role_selection,
            select_role_tx,
            set_reward_flag,
            set_user_wallet_address,
            set_wallet_address,
            token_validation,
            unlock_tokens_tx,
            update_lock_details,
            update_patron_status,
            update_user_role,
            withdraw_vesting_tx,
            // TCE endpoints
            get_tce_status,
            start_tce_tx,
            sync_rewards_to_chain,
            get_pending_rewards,
            update_user_accumulated_rewards_tx,
            update_tce_status
        },
    },
    state::AppState,
};
use axum::{
    routing::{get, post},
    Router,
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
        .route("/batch_claim_tx", post(batch_claim_tx))
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
        // TCE (Token Claim Event) endpoints
        .route("/tce_status", get(get_tce_status))
        .route("/start_tce", post(start_tce_tx))
        .route("/update_tce_status", post(update_tce_status))
        .route("/sync_rewards", post(sync_rewards_to_chain))
        .route("/pending_rewards", get(get_pending_rewards))
        // OTC swap endpoints
        .route("/initiate_otc_swap", post(initiate_otc_swap_tx))
        .route(
            "/initiate_otc_swap_enhanced",
            post(initiate_otc_swap_enhanced_tx),
        )
        .route("/accept_otc_swap", post(accept_otc_swap_tx))
        .route("/cancel_otc_swap", post(cancel_otc_swap_tx))
        .route("/process_cancel_otc_swap", post(process_cancel_otc_swap_tx))
        .route(
            "/update_otc_swap_signature",
            post(update_otc_swap_tx_signature),
        )
        .route("/swap_stats", get(get_swap_stats))
        .route("/swap/{pda}", get(get_swap_by_pda))
        // Tweet template endpoints (must be before user_id routes to avoid conflicts)
        .route("/tweet_templates", get(get_tweet_templates))
        .route("/post_tweet", post(post_tweet))
        .route("/fresh_templates", get(get_fresh_templates))
        // User ID specific routes (must be at the end to avoid conflicts)
        .route("/{user_id}", get(get_user_profile))
        .route("/{user_id}/mining_status", get(get_user_mining_status))
        .route("/{user_id}/wallet", post(set_user_wallet_address))
        .route("/{user_id}/patron_status", post(update_patron_status))
        .route("/{user_id}/role", post(update_user_role))
        .route("/{user_id}/lock_details", post(update_lock_details))
        .route("/{user_id}/phase2_tweets", get(get_user_phase2_tweets))
        .route("/{user_id}/update_rewards", post(update_user_accumulated_rewards_tx))
    }
