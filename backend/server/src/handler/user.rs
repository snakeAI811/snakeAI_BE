use std::str::FromStr;

use crate::state::AppState;
use anchor_client::solana_sdk::{
    message::Message, pubkey::Pubkey, signature::Keypair, signer::Signer, system_program,
    transaction::Transaction,
};
use axum::{
    Extension, Json,
    extract::{Query, State, Path},
};
use base64::{Engine, engine};
use serde_json::{json, Value};
use types::{
    dto::{GetRewardsQuery, GetTweetsQuery, SetWalletAddressRequest},
    error::{ApiError, ValidatedRequest},
    model::{Profile, RewardWithUserAndTweet, TweetWithUser, User},
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdatePatronStatusRequest {
    pub patron_status: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRoleRequest {
    pub selected_role: String,
}

#[derive(Deserialize)]
pub struct UpdateLockDetailsRequest {
    pub lock_duration_months: i32,
    pub locked_amount: i64,
}

#[derive(Deserialize)]
pub struct SetWalletAddressForUserRequest {
    pub wallet_address: String,
}

pub async fn token_validation(Extension(_): Extension<User>) -> Result<Json<bool>, ApiError> {
    Ok(Json(true))
}

pub async fn get_me(Extension(user): Extension<User>) -> Result<Json<User>, ApiError> {
    Ok(Json(user))
}

pub async fn get_profile(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<Json<Profile>, ApiError> {
    let reward_balance = state.service.reward.get_reward_balance(&user.id).await?;
    let tweets = state.service.tweet.get_tweets_count().await?;
    Ok(Json(Profile {
        twitter_username: user.twitter_username.unwrap_or_default(),
        wallet_address: user.wallet_address.unwrap_or_default(),
        latest_claim_timestamp: user.latest_claim_timestamp,
        reward_balance,
        tweets,
        likes: 0,
        replies: 0,
    }))
}

pub async fn get_mining_status(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<Json<Value>, ApiError> {
    let phase2_mining_count = state.service.tweet.get_phase2_mining_count(&user.id).await?;
    let phase1_mining_count = state.service.tweet.get_phase1_mining_count(&user.id).await?;
    let total_mining_count = phase1_mining_count + phase2_mining_count;
    let current_phase = state.env.get_mining_phase();
    let is_phase2 = state.env.is_phase2();
    
    Ok(Json(json!({
        "phase1_mining_count": phase1_mining_count,
        "phase2_mining_count": phase2_mining_count,
        "total_mining_count": total_mining_count,
        "current_phase": current_phase,
        "is_phase2": is_phase2,
        "phase2_start_date": state.env.phase2_start_date,
        "user_id": user.id,
        "wallet_address": user.wallet_address
    })))
}

// New endpoint for getting mining status by user ID
pub async fn get_user_mining_status(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| ApiError::BadRequest("Invalid user ID format".to_string()))?;
    
    let user = state.service.user.get_user_by_id(&user_uuid).await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;
    
    let phase2_mining_count = state.service.tweet.get_phase2_mining_count(&user.id).await?;
    let phase1_mining_count = state.service.tweet.get_phase1_mining_count(&user.id).await?;
    let total_mining_count = phase1_mining_count + phase2_mining_count;
    let current_phase = state.env.get_mining_phase();
    let is_phase2 = state.env.is_phase2();
    
    Ok(Json(json!({
        "phase1_mining_count": phase1_mining_count,
        "phase2_mining_count": phase2_mining_count,
        "total_mining_count": total_mining_count,
        "current_phase": current_phase,
        "is_phase2": is_phase2,
        "phase2_start_date": state.env.phase2_start_date,
        "wallet_address": user.wallet_address
    })))
}

// Get user profile by user ID
pub async fn get_user_profile(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| ApiError::BadRequest("Invalid user ID format".to_string()))?;
    
    let user = state.service.user.get_user_by_id(&user_uuid).await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;
    
    Ok(Json(json!({
        "id": user.id,
        "username": user.twitter_username.unwrap_or_default(),
        "wallet_address": user.wallet_address,
        "patron_status": user.patron_status,
        "selected_role": user.selected_role,
        "lock_duration_months": user.lock_duration_months,
        "locked_amount": user.locked_amount,
        "patron_qualification_score": user.patron_qualification_score,
        "wallet_age_days": user.wallet_age_days,
        "community_score": user.community_score,
        "created_at": user.created_at,
        "updated_at": user.updated_at
    })))
}

// Set wallet address for a user
pub async fn set_user_wallet_address(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<SetWalletAddressForUserRequest>,
) -> Result<Json<Value>, ApiError> {
    // Validate wallet address format
    Pubkey::from_str(&payload.wallet_address)
        .map_err(|err| ApiError::BadRequest(err.to_string()))?;

    // Check if wallet is already in use
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

    // Parse user_id to UUID
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| ApiError::BadRequest("Invalid user ID format".to_string()))?;

    // Update user wallet address
    let _user = state
        .service
        .user
        .set_wallet_address(&user_uuid, &payload.wallet_address)
        .await?;

    Ok(Json(json!({ "success": true })))
}

// Update patron status
pub async fn update_patron_status(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<UpdatePatronStatusRequest>,
) -> Result<Json<Value>, ApiError> {
    // Validate patron status
    if !["none", "applied", "approved", "rejected"].contains(&payload.patron_status.as_str()) {
        return Err(ApiError::BadRequest("Invalid patron status".to_string()));
    }

    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| ApiError::BadRequest("Invalid user ID format".to_string()))?;

    let _user = state
        .service
        .user
        .update_patron_status(&user_uuid, &payload.patron_status)
        .await?;

    Ok(Json(json!({ "success": true })))
}

// Update user role
pub async fn update_user_role(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRoleRequest>,
) -> Result<Json<Value>, ApiError> {
    // Validate role
    if !["none", "staker", "patron"].contains(&payload.selected_role.as_str()) {
        return Err(ApiError::BadRequest("Invalid role".to_string()));
    }

    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| ApiError::BadRequest("Invalid user ID format".to_string()))?;

    let _user = state
        .service
        .user
        .update_selected_role(&user_uuid, &payload.selected_role)
        .await?;

    Ok(Json(json!({ "success": true })))
}

// Update lock details
pub async fn update_lock_details(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateLockDetailsRequest>,
) -> Result<Json<Value>, ApiError> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| ApiError::BadRequest("Invalid user ID format".to_string()))?;

    let _user = state
        .service
        .user
        .update_lock_details(&user_uuid, payload.lock_duration_months, payload.locked_amount)
        .await?;

    Ok(Json(json!({ "success": true })))
}

// Get Phase 2 tweets for a user
pub async fn get_user_phase2_tweets(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<Value>>, ApiError> {
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| ApiError::BadRequest("Invalid user ID format".to_string()))?;
    
    let tweets = state.service.tweet.get_tweets_by_phase(&Some(user_uuid), "Phase2").await?;
    
    let tweet_values: Vec<Value> = tweets.into_iter().map(|tweet| {
        let phase_string = match tweet.mining_phase {
            Some(2) => "Phase2",
            Some(1) => "Phase1", 
            _ => "Phase1"
        };
        
        json!({
            "id": tweet.id,
            "content": "Tweet content", // Note: tweets table doesn't have content field
            "created_at": tweet.created_at,
            "mining_phase": phase_string
        })
    }).collect();

    Ok(Json(tweet_values))
}

pub async fn set_wallet_address(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    ValidatedRequest(payload): ValidatedRequest<SetWalletAddressRequest>,
) -> Result<Json<User>, ApiError> {
    if user.wallet().is_some() {
        return Err(ApiError::BadRequest(
            "Wallet address is already set".to_string(),
        ));
    }

    Pubkey::from_str(&payload.wallet_address)
        .map_err(|err| ApiError::BadRequest(err.to_string()))?;

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
        .set_wallet_address_by_uuid(&user.id, &payload.wallet_address)
        .await?;

    Ok(Json(user))
}

pub async fn get_rewards(
    Extension(user): Extension<User>,
    Query(opts): Query<GetRewardsQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<RewardWithUserAndTweet>>, ApiError> {
    let rewards = state
        .service
        .reward
        .get_rewards(&Some(user.id), opts.offset, opts.limit, opts.available)
        .await?;

    Ok(Json(rewards))
}

pub async fn get_tweets(
    Extension(user): Extension<User>,
    Query(opts): Query<GetTweetsQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<TweetWithUser>>, ApiError> {
    let user_id = if user.twitter_id == state.env.play_snake_ai_id {
        None
    } else {
        Some(user.id)
    };

    let tweets = state
        .service
        .tweet
        .get_tweets(&user_id, opts.offset, opts.limit)
        .await?;

    Ok(Json(tweets))
}

const REWARD_POOL_SEED: &[u8] = b"reward_pool";
const USER_CLAIM_SEED: &[u8] = b"user_claim";

// Get token information for the user
pub async fn get_token_info(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    let reward_balance = state.service.reward.get_reward_balance(&user.id).await?;
    let mining_status = state.service.tweet.get_phase1_mining_count(&user.id).await? 
        + state.service.tweet.get_phase2_mining_count(&user.id).await?;
    
    // In a real implementation, you would get locked, staked amounts from smart contract
    Ok(Json(json!({
        "balance": reward_balance,
        "locked": user.locked_amount.unwrap_or(0),
        "staked": 0, // Would come from smart contract
        "rewards": reward_balance,
        "mining_count": mining_status,
        "lockEndDate": user.latest_claim_timestamp
    })))
}

// Get patron application status
pub async fn get_patron_application_status(
    Extension(user): Extension<User>,
    State(_state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(json!({
        "id": user.id,
        "status": user.patron_status.unwrap_or_else(|| "none".to_string()),
        "submitted_at": user.created_at,
        "qualification_score": user.patron_qualification_score.unwrap_or(0)
    })))
}

// Get active OTC swaps
pub async fn get_active_swaps(
    Extension(_user): Extension<User>,
    State(_state): State<AppState>,
) -> Result<Json<Vec<Value>>, ApiError> {
    // In a real implementation, this would query the database for active OTC swaps
    // For now, return empty array as placeholder
    Ok(Json(vec![]))
}

// Get user's OTC swaps
pub async fn get_my_swaps(
    Extension(_user): Extension<User>,
    State(_state): State<AppState>,
) -> Result<Json<Vec<Value>>, ApiError> {
    // In a real implementation, this would query user's OTC swap history
    Ok(Json(vec![]))
}

// Get vesting information
pub async fn get_vesting_info(
    Extension(user): Extension<User>,
    State(_state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(json!({
        "amount": user.locked_amount.unwrap_or(0),
        "role_type": user.selected_role.unwrap_or_else(|| "none".to_string()),
        "created_at": user.created_at,
        "lock_duration_months": user.lock_duration_months.unwrap_or(0),
        "withdrawal_available": false // Would be calculated based on lock period
    })))
}

pub async fn get_claim_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    if state
        .service
        .reward
        .get_available_reward(&user.id)
        .await?
        .is_some()
    {
        if let Some(wallet) = user.wallet() {
            let admin = Keypair::from_base58_string(&state.env.backend_wallet_private_key);
            let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
            let (reward_pool, _) =
                Pubkey::find_program_address(&[REWARD_POOL_SEED], &state.program.id());
            let treasury =
                spl_associated_token_account::get_associated_token_address(&reward_pool, &mint);
            let (user_claim, _) = Pubkey::find_program_address(
                &[USER_CLAIM_SEED, wallet.as_array()],
                &state.program.id(),
            );
            let user_token_ata =
                spl_associated_token_account::get_associated_token_address(&wallet, &mint);

            let instructions = match state
                .program
                .request()
                .accounts(snake_contract::accounts::ClaimReward {
                    user: wallet,
                    admin: admin.pubkey(),
                    reward_pool,
                    treasury,
                    user_claim,
                    user_token_ata,
                    mint,
                    associated_token_program: spl_associated_token_account::ID,
                    token_program: spl_token::ID,
                    system_program: system_program::ID,
                })
                .args(snake_contract::instruction::ClaimReward)
                .instructions()
            {
                Ok(ixs) => ixs,
                Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
            };

            let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
                Ok(latest_blockhash) => latest_blockhash,
                Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
            };

            let message = Message::new(&instructions, Some(&wallet));
            let mut transaction = Transaction::new_unsigned(message);
            transaction.partial_sign(&[&admin], _latest_blockhash);
            let serialized_transaction = bincode::serialize(&transaction).unwrap();
            let base64_transaction =
                engine::general_purpose::STANDARD.encode(&serialized_transaction);

            Ok(Json(base64_transaction))
        } else {
            Err(ApiError::BadRequest("User didn't set wallet".to_string()))
        }
    } else {
        Err(ApiError::BadRequest("No available reward".to_string()))
    }
}

// ========== SMART CONTRACT INTERACTION ENDPOINTS ==========

#[derive(Deserialize)]
pub struct SelectRoleRequest {
    pub role: String, // "none", "staker", "patron"
}

#[derive(Deserialize)]
pub struct ClaimTokensRequest {
    pub amount: u64,
    pub role: String,
}

#[derive(Deserialize)]
pub struct PatronApplicationRequest {
    pub wallet_age_days: u32,
    pub community_score: u32,
}

#[derive(Deserialize)]
pub struct ApprovePatronRequest {
    pub min_qualification_score: u32,
}

#[derive(Deserialize)]
pub struct LockTokensRequest {
    pub amount: u64,
    pub duration_months: u8,
}

#[derive(Deserialize)]
pub struct VestingRequest {
    pub amount: u64,
    pub role_type: String, // "staker", "patron"
}

#[derive(Deserialize)]
pub struct InitiateOtcSwapRequest {
    pub token_amount: u64,
    pub sol_rate: u64,
    pub buyer_rebate: u64,
    pub buyer_role_required: String, // "none", "staker", "patron"
}

#[derive(Deserialize)]
pub struct AcceptOtcSwapRequest {
    pub seller_pubkey: String,
}

// const PATRON_APPLICATION_SEED: &[u8] = b"patron_application";
// const TOKEN_LOCK_SEED: &[u8] = b"token_lock";
const VESTING_SEED: &[u8] = b"vesting";
const OTC_SWAP_SEED: &[u8] = b"otc_swap";
// const ESCROW_SEED: &[u8] = b"escrow";

/// Select user role (None, Staker, Patron)
pub async fn select_role_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<SelectRoleRequest>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    // Validate role
    let role = match payload.role.as_str() {
        "none" => snake_contract::state::UserRole::None,
        "staker" => snake_contract::state::UserRole::Staker,
        "patron" => snake_contract::state::UserRole::Patron,
        _ => return Err(ApiError::BadRequest("Invalid role".to_string())),
    };

    let (user_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::SelectRole {
            user: wallet,
            user_claim,
        })
        .args(snake_contract::instruction::SelectRole {
            role,
        })
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&wallet));
    let transaction = Transaction::new_unsigned(message);
    
    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

/// Apply for patron status
pub async fn apply_patron_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<PatronApplicationRequest>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    let (user_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::ApplyForPatron {
            user: wallet,
            user_claim,
        })
        .args(snake_contract::instruction::ApplyForPatron {
            wallet_age_days: payload.wallet_age_days,
            community_score: payload.community_score,
        })
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&wallet));
    let transaction = Transaction::new_unsigned(message);
    
    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

/// Approve patron application (admin only)
pub async fn approve_patron_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<ApprovePatronRequest>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    let admin = Keypair::from_base58_string(&state.env.backend_wallet_private_key);
    let (user_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::ApprovePatronApplication {
            admin: admin.pubkey(),
            applicant: wallet,
            user_claim,
        })
        .args(snake_contract::instruction::ApprovePatronApplication {
            min_qualification_score: payload.min_qualification_score,
        })
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&admin.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.partial_sign(&[&admin], _latest_blockhash);
    
    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

/// Claim tokens with role-specific logic
pub async fn claim_tokens_with_role_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<ClaimTokensRequest>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    // Validate role
    let role = match payload.role.as_str() {
        "none" => snake_contract::state::UserRole::None,
        "staker" => snake_contract::state::UserRole::Staker,
        "patron" => snake_contract::state::UserRole::Patron,
        _ => return Err(ApiError::BadRequest("Invalid role".to_string())),
    };

    let admin = Keypair::from_base58_string(&state.env.backend_wallet_private_key);
    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    let (reward_pool, _) = Pubkey::find_program_address(&[REWARD_POOL_SEED], &state.program.id());
    let treasury = spl_associated_token_account::get_associated_token_address(&reward_pool, &mint);
    let (user_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );
    let user_token_ata = spl_associated_token_account::get_associated_token_address(&wallet, &mint);

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::ClaimTokensWithRole {
            user: wallet,
            user_claim,
            user_token_ata,
            reward_pool_pda: reward_pool,
            treasury_token_account: treasury,
            mint,
            token_program: spl_token::ID,
        })
        .args(snake_contract::instruction::ClaimTokensWithRole {
            amount: payload.amount,
            role,
        })
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&wallet));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.partial_sign(&[&admin], _latest_blockhash);
    
    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

/// Lock tokens for staking
pub async fn lock_tokens_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<LockTokensRequest>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    let (user_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );
    let user_token_ata = spl_associated_token_account::get_associated_token_address(&wallet, &mint);
    let (reward_pool, _) = Pubkey::find_program_address(&[REWARD_POOL_SEED], &state.program.id());
    let treasury = spl_associated_token_account::get_associated_token_address(&reward_pool, &mint);

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::LockTokens {
            user: wallet,
            user_claim,
            user_token_account: user_token_ata,
            reward_pool_pda: reward_pool,
            treasury_token_account: treasury,
            token_program: spl_token::ID,
        })
        .args(snake_contract::instruction::LockTokens {
            amount: payload.amount,
            duration_months: payload.duration_months,
        })
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&wallet));
    let transaction = Transaction::new_unsigned(message);
    
    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

/// Unlock tokens after lock period
pub async fn unlock_tokens_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    let (user_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );
    let user_token_ata = spl_associated_token_account::get_associated_token_address(&wallet, &mint);
    let (reward_pool, _) = Pubkey::find_program_address(&[REWARD_POOL_SEED], &state.program.id());
    let treasury = spl_associated_token_account::get_associated_token_address(&reward_pool, &mint);

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::UnlockTokens {
            user: wallet,
            user_claim,
            user_token_account: user_token_ata,
            reward_pool_pda: reward_pool,
            treasury_token_account: treasury,
            token_program: spl_token::ID,
        })
        .args(snake_contract::instruction::UnlockTokens {})
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&wallet));
    let transaction = Transaction::new_unsigned(message);
    
    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

/// Claim staking yield
pub async fn claim_yield_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    let (reward_pool, _) = Pubkey::find_program_address(&[REWARD_POOL_SEED], &state.program.id());
    // let treasury = spl_associated_token_account::get_associated_token_address(&reward_pool, &mint);
    let (user_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );
    let user_token_ata = spl_associated_token_account::get_associated_token_address(&wallet, &mint);

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::ClaimYield {
            user: wallet,
            user_claim,
            user_token_account: user_token_ata,
            mint,
            reward_pool_pda: reward_pool,
            token_program: spl_token::ID,
        })
        .args(snake_contract::instruction::ClaimYield {})
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&wallet));
    let transaction = Transaction::new_unsigned(message);
    
    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

/// Create vesting schedule
pub async fn create_vesting_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<VestingRequest>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    // Validate role type
    // let role_type = match payload.role_type.as_str() {
    //     "staker" => snake_contract::state::VestingRoleType::Staker,
    //     "patron" => snake_contract::state::VestingRoleType::Patron,
    //     _ => return Err(ApiError::BadRequest("Invalid role type".to_string())),
    // };

    let admin = Keypair::from_base58_string(&state.env.backend_wallet_private_key);
    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    let user_token_ata = spl_associated_token_account::get_associated_token_address(&wallet, &mint);
    let (user_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );
    let (vesting_account, _) = Pubkey::find_program_address(
        &[VESTING_SEED, wallet.as_array()],
        &state.program.id(),
    );
    let escrow_vault = spl_associated_token_account::get_associated_token_address(&vesting_account, &mint);

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::CreateVesting {
            user: wallet,
            user_claim,
            user_token_account: user_token_ata,
            vesting_schedule: vesting_account,
            vesting_escrow: escrow_vault,
            system_program: system_program::ID,
            token_program: spl_token::ID,
        })
        .args(snake_contract::instruction::CreateVestingSchedule {
            vesting_amount: payload.amount,
        })
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&wallet));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.partial_sign(&[&admin], _latest_blockhash);
    
    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

/// Withdraw from vesting
pub async fn withdraw_vesting_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    let admin = Keypair::from_base58_string(&state.env.backend_wallet_private_key);
    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    let user_token_ata = spl_associated_token_account::get_associated_token_address(&wallet, &mint);
    let (user_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );
    let (vesting_account, _) = Pubkey::find_program_address(
        &[VESTING_SEED, wallet.as_array()],
        &state.program.id(),
    );
    let escrow_vault = spl_associated_token_account::get_associated_token_address(&vesting_account, &mint);
    let (reward_pool, _) = Pubkey::find_program_address(&[REWARD_POOL_SEED], &state.program.id());
    let _treasury = spl_associated_token_account::get_associated_token_address(&reward_pool, &mint);

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::WithdrawVesting {
            user: wallet,
            user_claim,
            vesting_schedule: vesting_account,
            vesting_escrow: escrow_vault,
            user_token_account: user_token_ata,
            token_program: spl_token::ID,
        })
        .args(snake_contract::instruction::ClaimVestedTokens {})
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&wallet));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.partial_sign(&[&admin], _latest_blockhash);
    
    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

/// Initiate OTC swap
pub async fn initiate_otc_swap_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<InitiateOtcSwapRequest>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    // let admin = Keypair::from_base58_string(&state.env.backend_wallet_private_key);
    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    let seller_token_ata = spl_associated_token_account::get_associated_token_address(&wallet, &mint);
    let (user_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );
    let (otc_swap, _) = Pubkey::find_program_address(
        &[OTC_SWAP_SEED, wallet.as_array()],
        &state.program.id(),
    );

    // Convert role string to enum
    let buyer_role_required = match payload.buyer_role_required.as_str() {
        "none" => snake_contract::state::UserRole::None,
        "staker" => snake_contract::state::UserRole::Staker,
        "patron" => snake_contract::state::UserRole::Patron,
        _ => return Err(ApiError::BadRequest("Invalid buyer role".to_string())),
    };

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::InitiateOtcSwap {
            seller: wallet,
            seller_claim: user_claim,
            otc_swap,
            seller_token_account: seller_token_ata,
            token_program: spl_token::ID,
            system_program: system_program::ID,
        })
        .args(snake_contract::instruction::InitiateOtcSwap {
            amount: payload.token_amount,
            rate: payload.sol_rate,
            buyer_rebate: payload.buyer_rebate,
            buyer_role_required,
        })
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&wallet));
    let transaction = Transaction::new_unsigned(message);
    
    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

/// Accept OTC swap
pub async fn accept_otc_swap_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<AcceptOtcSwapRequest>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    let admin = Keypair::from_base58_string(&state.env.backend_wallet_private_key);
    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    let seller_pubkey = Pubkey::from_str(&payload.seller_pubkey)
        .map_err(|_| ApiError::BadRequest("Invalid seller pubkey".to_string()))?;
    
    let buyer_token_ata = spl_associated_token_account::get_associated_token_address(&wallet, &mint);
    let seller_token_ata = spl_associated_token_account::get_associated_token_address(&seller_pubkey, &mint);
    let (buyer_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );
    let (otc_swap, _) = Pubkey::find_program_address(
        &[OTC_SWAP_SEED, seller_pubkey.as_array()],
        &state.program.id(),
    );
    let (treasury, _) = Pubkey::find_program_address(
        &[b"treasury"],
        &state.program.id(),
    );

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::AcceptOtcSwap {
            buyer: wallet,
            buyer_claim,
            seller: seller_pubkey,
            otc_swap,
            seller_token_account: seller_token_ata,
            buyer_token_account: buyer_token_ata,
            treasury,
            token_program: spl_token::ID,
            system_program: system_program::ID,
        })
        .args(snake_contract::instruction::AcceptOtcSwap {})
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&wallet));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.partial_sign(&[&admin], _latest_blockhash);
    
    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

/// Cancel OTC swap
pub async fn cancel_otc_swap_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    let admin = Keypair::from_base58_string(&state.env.backend_wallet_private_key);
    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    let seller_token_ata = spl_associated_token_account::get_associated_token_address(&wallet, &mint);
    let (otc_swap, _) = Pubkey::find_program_address(
        &[OTC_SWAP_SEED, wallet.as_array()],
        &state.program.id(),
    );

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::CancelOtcSwap {
            seller: wallet,
            otc_swap,
            seller_token_account: seller_token_ata,
            token_program: spl_token::ID,
        })
        .args(snake_contract::instruction::CancelOtcSwap {})
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&wallet));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.partial_sign(&[&admin], _latest_blockhash);
    
    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}
