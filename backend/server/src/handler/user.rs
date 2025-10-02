use std::str::FromStr;
use std::sync::{Arc, Mutex};
use crate::state::AppState;
use anchor_client::{
    anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas},
    solana_sdk::{
        message::Message, pubkey::Pubkey, signature::Keypair, signer::Signer, system_program,
        transaction::Transaction,
        instruction::Instruction
    },
};
use axum::{
    Extension, Json,
    extract::{Query, State, Path},
};
use base64::{Engine, engine};
use serde_json::{json, Value};
use types::{
    dto::{GetRewardsQuery, GetTweetsQuery, SetWalletAddressRequest, SetRewardFlagRequest, TweetMiningStatusResponse},
    error::{ApiError, ValidatedRequest},
    model::{Profile, RewardWithUserAndTweet, TweetWithUser, User},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::services::{MiningPhase, get_current_mining_phase};
use spl_associated_token_account::ID as ASSOCIATED_TOKEN_PROGRAM_ID;

#[derive(Deserialize)]
pub struct UpdatePatronStatusRequest {
    pub patron_status: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRoleRequest {
    pub role: String,
}

#[derive(Deserialize)]
pub struct PatronEligibilityRequest {
    pub token_amount: u64,
    pub wallet_age_days: u32,
    pub total_mined_phase1: u64,
}

// Import constants from smart contract
use snake_contract::constants::{
    REWARD_POOL_SEED,
    USER_CLAIM_SEED,
    VESTING_SEED,
    PATRON_MIN_TOKEN_AMOUNT, 
    PATRON_MIN_WALLET_AGE_DAYS, 
    PATRON_MIN_STAKING_MONTHS, 
    LAMPORTS_PER_SNK
};

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
    let claimable_rewards = state.service.reward.count_available_rewards(&user.id).await?;

    let tweets = state.service.tweet.get_tweets_count(Some(user.id)).await?;
    Ok(Json(Profile {
        twitter_username: user.twitter_username.unwrap_or_default(),
        wallet_address: user.wallet_address.unwrap_or_default(),
        latest_claim_timestamp: user.latest_claim_timestamp,
        reward_balance,
        claimable_rewards,
        tweets,
        likes: 0,
        replies: 0,
        accumulated_reward: user.accumulated_reward.unwrap_or(0),
    }))
}

pub async fn get_mining_status(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<Json<Value>, ApiError> {
    let phase2_mining_count = state.service.tweet.get_all_phase2_mining_count().await?;
    let phase1_mining_count = state.service.tweet.get_all_phase1_mining_count().await?;
    let total_mining_count = phase1_mining_count + phase2_mining_count;

    let mining_phase = get_current_mining_phase(phase1_mining_count);
    let is_phase2 = if mining_phase == MiningPhase::Phase2 { true } else { false }; 
    
    Ok(Json(json!({
        "phase1_mining_count": phase1_mining_count,
        "phase2_mining_count": phase2_mining_count,
        "total_mining_count": total_mining_count,
        "current_phase": if mining_phase == MiningPhase::Phase2 { 2 } else { 1 },
        "is_phase2": is_phase2,
        // "phase2_start_date": state.env.phase2_start_date,
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

    let phase1_mining_count_all = state.service.tweet.get_all_phase1_mining_count().await?;
    let mining_phase = get_current_mining_phase(phase1_mining_count_all);
    // let current_phase = state.env.get_mining_phase();
    // let is_phase2 = state.env.is_phase2();
    
    Ok(Json(json!({
        "phase1_mining_count": phase1_mining_count,
        "phase2_mining_count": phase2_mining_count,
        "total_mining_count": total_mining_count,
        "current_phase": if mining_phase == MiningPhase::Phase2 { 2 } else { 1 },
        // "is_phase2": is_phase2,
        // "phase2_start_date": state.env.phase2_start_date,
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
        "role": user.role,
        "lock_duration_months": user.lock_duration_months,
        "locked_amount": user.locked_amount,
        "patron_qualification_score": user.patron_qualification_score,
        "wallet_age_days": user.wallet_age_days,
        "community_score": user.community_score,
        "accumulated_reward": user.accumulated_reward.unwrap_or(0),
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
    if !["none", "staker", "patron"].contains(&payload.role.as_str()) {
        return Err(ApiError::BadRequest("Invalid role".to_string()));
    }

    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| ApiError::BadRequest("Invalid user ID format".to_string()))?;

    let _user = state
        .service
        .user
        .update_role(&user_uuid, &payload.role)
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
pub async fn get_user_phase2_tweets (
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

pub async fn set_reward_flag(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    ValidatedRequest(payload): ValidatedRequest<SetRewardFlagRequest>,
) -> Result<Json<bool>, ApiError> {
    // Validate tweet ID format
    if payload.tweet_id.is_empty() {
        return Err(ApiError::BadRequest("Tweet ID cannot be empty".to_string()));
    }

    // Set reward flag for the tweet
    let success = state
        .service
        .reward
        .set_reward_flag(&user.id, &payload.tweet_id)
        .await?;

    Ok(Json(success))
}

pub async fn get_tweets(
    Extension(user): Extension<User>,
    Query(opts): Query<GetTweetsQuery>,
    State(state): State<AppState>,
) -> Result<Json<Vec<TweetWithUser>>, ApiError> {
    // let user_id = if user.twitter_id == state.env.play_snake_ai_id {
    //     None
    // } else {
    //     Some(user.id)
    // };

    let user_id = Some(user.id);

    let tweets = state
        .service
        .tweet
        .get_tweets(&user_id, opts.offset, opts.limit)
        .await?;

    Ok(Json(tweets))
}

// Get token information for the user
pub async fn get_token_info(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    let mining_status = state.service.tweet.get_phase1_mining_count(&user.id).await?
        + state.service.tweet.get_phase2_mining_count(&user.id).await?;

    if let Some(wallet_address) = &user.wallet_address {
        let wallet = Pubkey::from_str(wallet_address)
            .map_err(|_| ApiError::BadRequest("Invalid wallet address".to_string()))?;
        let mint = Pubkey::from_str(&state.env.token_mint)
            .map_err(|_| ApiError::InternalServerError("Invalid token mint".to_string()))?;

        // Get associated token account address
        let user_token_ata = spl_associated_token_account::get_associated_token_address(&wallet, &mint);
        log::info!("Checking token account: {} for wallet: {}", user_token_ata, wallet);

        // Get token account balance from blockchain
        let (wallet_balance_raw, token_decimals) = {
            let mut attempts = 0;
            let max_attempts = 3;
            let mut balance = 0u64;
            let mut decimals = 9u8; // Default to 9 decimals

            while attempts < max_attempts {
                match state.program.rpc().get_token_account_balance(&user_token_ata) {
                    Ok(balance_response) => {
                        // Use raw amount to maintain precision
                        balance = balance_response.amount.parse::<u64>().unwrap_or(0);
                        decimals = balance_response.decimals;
                        
                        let ui_amount = balance as f64 / 10_f64.powi(decimals as i32);
                        log::info!("Successfully retrieved balance: {} raw units = {} tokens (decimals: {}, attempt: {})", 
                                  balance, ui_amount, decimals, attempts + 1);
                        break;
                    },
                    Err(e) => {
                        attempts += 1;
                        log::warn!("Failed to get token account balance (attempt {}): {:?}", attempts, e);
                        if attempts < max_attempts {
                            // Use async sleep instead of thread sleep
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                        } else {
                            log::error!("Failed to get token balance after {} attempts", max_attempts);
                        }
                    }
                }
            }
            (balance, decimals)
        };

        // Get staking/lock information from smart contract
        // ✅ FIXED: Use consistent seed derivation with as_ref()
        let (user_claim, _) = Pubkey::find_program_address(
            &[USER_CLAIM_SEED, wallet.as_ref()], // Changed from as_array() to as_ref()
            &state.program.id(),
        );

        log::info!("Checking UserClaim account at address: {} for wallet: {}", user_claim, wallet);

        let (locked_amount, lock_end_timestamp, yield_rewards, user_role, apy_rate, lock_duration) = match state.program.rpc().get_account_data(&user_claim) {
            Ok(data) => {
                log::info!("UserClaim account found, data length: {} bytes", data.len());
                // Try to deserialize the UserClaim account data
                match snake_contract::state::UserClaim::try_deserialize(&mut data.as_slice()) {
                    Ok(user_claim_data) => {
                        let role_str = match user_claim_data.role {
                            snake_contract::state::UserRole::Staker => "Staker",
                            snake_contract::state::UserRole::Patron => "Patron",
                            snake_contract::state::UserRole::None => "None",
                        };

                        // ✅ FIXED: Get APY rate based on role
                        let apy_rate = match user_claim_data.role {
                            snake_contract::state::UserRole::Staker => 5u8,  // 5% APY
                            snake_contract::state::UserRole::Patron => 7u8,  // 7% APY
                            _ => 0u8,
                        };
                        
                       // Replace this section in your get_token_info function:

                        let current_time = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as i64;

                        log::debug!("UserClaim deserialized successfully - locked_amount: {}, lock_end: {}, role: {}, APY: {}%, current_time: {}",  
                                user_claim_data.locked_amount, user_claim_data.lock_end_timestamp, role_str, apy_rate, current_time);

                        // ✅ FIXED: Use backend-compatible yield calculation
                        let yield_rewards = if (user_claim_data.role == snake_contract::state::UserRole::Staker  
                                            || user_claim_data.role == snake_contract::state::UserRole::Patron) 
                            && user_claim_data.locked_amount > 0 {
                            
                            // Use the new backend-compatible method
                            let calculated_yield = user_claim_data.calculate_yield_backend(current_time);
                            log::debug!("Calculated yield rewards: {} for role: {} ({}% APY)",  
                                    calculated_yield, role_str, apy_rate);
                            calculated_yield
                        } else { 
                            log::debug!("No yield rewards - role: {}, locked: {}, current_time: {}, lock_end: {}",  
                                    role_str, user_claim_data.locked_amount, current_time, user_claim_data.lock_end_timestamp); 
                            0 
                        };

                        (
                            user_claim_data.locked_amount, 
                            user_claim_data.lock_end_timestamp, 
                            yield_rewards,
                            role_str.to_string(),
                            apy_rate,
                            user_claim_data.lock_duration_months
                        )
                    },
                    Err(e) => {
                        log::warn!("Failed to deserialize UserClaim account for wallet {}: {:?}", wallet, e);
                        log::debug!("Raw account data: {:?}", data);
                        (0, 0, 0, "None".to_string(), 0u8, 0u8) // Account exists but can't deserialize
                    }
                }
            },
            Err(e) => {
                log::info!("UserClaim account not found for wallet {}: {:?}", wallet, e);
                (0, 0, 0, "None".to_string(), 0u8, 0u8) // Account doesn't exist
            }
        };

        // Get pending rewards from database (mining rewards)
        let mining_rewards = state.service.reward.get_reward_balance(&user.id).await?;
        log::info!("Mining rewards from database: {}", mining_rewards);

        // Total claimable rewards = yield rewards + mining rewards
        let total_rewards = yield_rewards + (mining_rewards.max(0) as u64);

        // Convert raw amounts to UI amounts for logging
        let wallet_balance_ui = wallet_balance_raw as f64 / 10_f64.powi(token_decimals as i32);
        let locked_amount_ui = locked_amount as f64 / 10_f64.powi(token_decimals as i32);
        let total_rewards_ui = total_rewards as f64 / 10_f64.powi(token_decimals as i32);
        let yield_rewards_ui = yield_rewards as f64 / 10_f64.powi(token_decimals as i32);

        log::info!("Final token info (UI amounts) - wallet_balance: {} tokens, locked: {} tokens, yield_rewards: {} tokens, mining_rewards: {}, total_rewards: {} tokens, role: {}, APY: {}%", 
                  wallet_balance_ui, locked_amount_ui, yield_rewards_ui, mining_rewards, total_rewards_ui, user_role, apy_rate);
        log::info!("Final token info (raw amounts) - wallet_balance: {}, locked: {}, yield_rewards: {}, total_rewards: {}", 
                  wallet_balance_raw, locked_amount, yield_rewards, total_rewards);

        // ✅ ENHANCED: Include more staking details in response
        Ok(Json(json!({
            "balance": wallet_balance_raw,           // Raw amount for precision
            "locked": locked_amount,                 // Raw amount for precision
            "staked": locked_amount,                 // Raw amount for precision
            "rewards": total_rewards,                // Raw amount for precision
            "yield_rewards": yield_rewards,          // Raw yield rewards separately
            "mining_rewards": mining_rewards.max(0),// Mining rewards from database
            "mining_count": mining_status,
            "lockEndDate": lock_end_timestamp,
            "decimals": token_decimals,              // Include decimals for frontend conversion
            
            // ✅ NEW: Staking details
            "staking": {
                "user_role": user_role,              // "Staker", "Patron", or "None"
                "apy_rate": apy_rate,                // 5% for Stakers, 7% for Patrons
                "lock_duration_months": lock_duration, // 3 or 6 months
                "is_locked": locked_amount > 0 && lock_end_timestamp > std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64,
                "can_claim_yield": yield_rewards > 0,
            },
            
            // Include UI amounts for easy display
            "balance_ui": wallet_balance_ui,
            "locked_ui": locked_amount_ui,
            "staked_ui": locked_amount_ui,
            "rewards_ui": total_rewards_ui,
            "yield_rewards_ui": yield_rewards_ui
        })))
    } else {
        log::info!("No wallet connected for user {}, returning database fallback values", user.id);
        // No wallet connected - return database values as fallback
        let pending_rewards = state.service.reward.get_reward_balance(&user.id).await?;
        let locked_amount = user.locked_amount.unwrap_or(0);
        
        // Assume 9 decimals for UI conversion when no wallet is connected
        let decimals = 9u8;
        let locked_amount_ui = locked_amount as f64 / 10_f64.powi(decimals as i32);
        let pending_rewards_ui = pending_rewards; //.max(0) as f64 / 10_f64.powi(decimals as i32);

        Ok(Json(json!({
            "balance": 0,
            "locked": locked_amount,
            "staked": 0,
            "rewards": pending_rewards.max(0),
            "yield_rewards": 0,
            "mining_rewards": pending_rewards.max(0),
            "mining_count": mining_status,
            "lockEndDate": user.latest_claim_timestamp,
            "decimals": decimals,
            
            // Default staking info for no wallet
            "staking": {
                "user_role": "None",
                "apy_rate": 0,
                "lock_duration_months": 0,
                "is_locked": false,
                "can_claim_yield": false,
            },
            
            // Include UI amounts for easy display
            "balance_ui": 0.0,
            "locked_ui": locked_amount_ui,
            "staked_ui": 0.0,
            "rewards_ui": pending_rewards_ui,
            "yield_rewards_ui": 0.0
        })))
    }
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

// Get vesting information
pub async fn get_vesting_info(
    Extension(user): Extension<User>,
    State(_state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(json!({
        "amount": user.locked_amount.unwrap_or(0),
        "role_type": user.role.unwrap_or_else(|| "none".to_string()),
        "created_at": user.created_at,
        "lock_duration_months": user.lock_duration_months.unwrap_or(0),
        "withdrawal_available": false // Would be calculated based on lock period
    })))
}

pub async fn batch_claim_tx(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<Json<Value>, ApiError> {
    let user_wallet = match user.wallet() {
        Some(wallet) => wallet,
        None => return Err(ApiError::BadRequest("User wallet not found".to_string())),
    };

    let mint = Pubkey::from_str(&state.env.token_mint)
        .map_err(|_| ApiError::BadRequest("Invalid token mint".to_string()))?;
    
    let (reward_pool, _) = Pubkey::find_program_address(&[REWARD_POOL_SEED], &state.program.id());
    let treasury = spl_associated_token_account::get_associated_token_address(&reward_pool, &mint);
    
    let (user_claim_pda, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, &user_wallet.to_bytes()],
        &state.program.id(),
    );

    let user_token_ata = spl_associated_token_account::get_associated_token_address(
        &user_wallet,
        &mint,
    );

    let instruction = state.program
        .request()
        .args(snake_contract::instruction::BatchClaim {})
        .accounts(snake_contract::accounts::BatchClaim {
            user: user_wallet,
            reward_pool,
            treasury,
            user_claim: user_claim_pda,
            user_token_ata,
            mint,
            associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
            token_program: spl_token::id(),
            system_program: system_program::id(),
        })
        .instructions()
        .map_err(|e| ApiError::InternalServerError(format!("Failed to create instruction: {}", e)))?;

    let recent_blockhash = state.program.rpc().get_latest_blockhash()
        .map_err(|e| ApiError::InternalServerError(format!("Failed to get latest blockhash: {}", e)))?;
    let admin = Keypair::from_base58_string(&state.env.backend_wallet_private_key);
    let tx = Transaction::new_signed_with_payer(
        &instruction,
        Some(&user_wallet),
        &[&admin],
        recent_blockhash,
    );

    let serialized_tx = bincode::serialize(&tx).unwrap();
    let base64_tx = engine::general_purpose::STANDARD.encode(serialized_tx);

    Ok(Json(json!({ "transaction": base64_tx })))
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
    pub tweet_id: Option<String>, // Optional for non-tweet claims
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
        &[USER_CLAIM_SEED, wallet.as_ref()],
        &state.program.id(),
    );

    let mut instructions = Vec::new();

    // Check if user_claim account exists, if not, add initialization instruction
    if state.program.rpc().get_account(&user_claim).is_err() {
        log::debug!("User claim account not found for wallet: {}, adding initialization instruction", wallet);
        let init_ix = Instruction {
            program_id: state.program.id(),
            accounts: snake_contract::accounts::InitializeUserClaim {
                user: wallet,
                user_claim,
                system_program: system_program::ID,
            }
            .to_account_metas(None),
            data: snake_contract::instruction::InitializeUserClaim {}.data(),
        };
        instructions.push(init_ix);
    } else {
        log::debug!("User claim account already exists for wallet: {}", wallet);
    }

    // Add select role instruction
    let select_role_instructions = match state
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
    
    instructions.extend(select_role_instructions);

    let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(hash) => hash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&wallet));
    let mut transaction = Transaction::new_unsigned(message);

    transaction.message.recent_blockhash = _latest_blockhash;
    
    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}


/// Check patron eligibility based on new requirements
pub async fn check_patron_eligibility(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<PatronEligibilityRequest>,
) -> Result<Json<Value>, ApiError> {
    let mut eligibility_status = json!({
        "eligible": false,
        "requirements": {
            "token_amount": {
                "required": PATRON_MIN_TOKEN_AMOUNT,
                "current": payload.token_amount,
                "met": payload.token_amount >= PATRON_MIN_TOKEN_AMOUNT
            },
            "wallet_age": {
                "required_days": PATRON_MIN_WALLET_AGE_DAYS,
                "current_days": payload.wallet_age_days,
                "met": payload.wallet_age_days >= PATRON_MIN_WALLET_AGE_DAYS
            },
            "mining_history": {
                "required": 1, // > 0
                "current": payload.total_mined_phase1,
                "met": payload.total_mined_phase1 > 0
            },
            "staking_history": {
                "required_months": PATRON_MIN_STAKING_MONTHS,
                "met": false, // Will be checked in next step
                "note": "6-month staking required (completed or in progress)"
            }
        },
        "errors": []
    });

    let mut errors = Vec::new();
    
    // Check token amount requirement
    if payload.token_amount < PATRON_MIN_TOKEN_AMOUNT {
        errors.push(format!("Insufficient token amount. Required: {} SNAKE, Current: {} SNAKE", 
            PATRON_MIN_TOKEN_AMOUNT / LAMPORTS_PER_SNK, payload.token_amount / LAMPORTS_PER_SNK));
    }
    
    // Check wallet age requirement
    if payload.wallet_age_days < PATRON_MIN_WALLET_AGE_DAYS {
        errors.push(format!("Wallet too young. Required: {} days, Current: {} days", 
            PATRON_MIN_WALLET_AGE_DAYS, payload.wallet_age_days));
    }
    
    // Check mining history requirement
    if payload.total_mined_phase1 == 0 {
        errors.push("No mining history found. You must have mined tokens in Phase 1".to_string());
    }

    // Note: 6-month staking history will be checked at the smart contract level
    // This endpoint only validates the basic requirements
    errors.push("6-month staking history validation will be performed during application submission".to_string());

    // Overall eligibility (excluding staking check which is done on-chain)
    let basic_requirements_met = payload.token_amount >= PATRON_MIN_TOKEN_AMOUNT 
        && payload.wallet_age_days >= PATRON_MIN_WALLET_AGE_DAYS 
        && payload.total_mined_phase1 > 0;
    
    let eligible = basic_requirements_met;
    eligibility_status["eligible"] = json!(eligible);
    eligibility_status["errors"] = json!(errors);

    Ok(Json(eligibility_status))
}

#[derive(Deserialize)]
pub struct SaveRoleSelectionRequest {
    pub role: String,
    pub transaction_signature: String,
    pub timestamp: String,
}

/// Save role selection to database after successful blockchain transaction
pub async fn save_role_selection(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<SaveRoleSelectionRequest>,
) -> Result<Json<String>, ApiError> {
    // Validate role
    let role = match payload.role.as_str() {
        "none" => "none",
        "staker" => "staker", 
        "patron" => "patron",
        _ => return Err(ApiError::BadRequest("Invalid role".to_string())),
    };

    // Update user role in database using the service
    match state.service.user.update_role(&user.id, role).await {
        Ok(_) => {
            log::debug!("✅ Role saved to database: {} for user {}", role, user.id);
            Ok(Json("Role saved successfully".to_string()))
        }
        Err(err) => {
            log::debug!("❌ Failed to save role to database: {}", err);
            Err(ApiError::InternalServerError(format!("Failed to save role to database: {}", err)))
        }
    }
}

// DAO-related endpoints

#[derive(Deserialize)]
pub struct DaoUsersQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
}

#[derive(serde::Serialize)]
pub struct DaoUser {
    pub id: String,
    pub username: String,
    pub wallet_address: String,
    pub score: i32,
    pub role_duration: i32,
    pub activity: i32,
    pub user_icon: String,
    pub avatar: Option<String>,
}

/// Get DAO users list with search and sorting

pub async fn get_dao_users(
    State(state): State<AppState>,
    Query(query): Query<DaoUsersQuery>,
    Extension(_user): Extension<User>,
) -> Result<Json<Vec<DaoUser>>, ApiError> {
    // Get user IDs who have at least one reward (i.e., started mining)
    let mining_user_ids = state.service.reward.get_distinct_user_ids_with_rewards().await?;
    let mut dao_users = Vec::new();
    for user_id in mining_user_ids {
        if let Some(user) = state.service.user.get_user_by_id(&user_id).await? {
            let mining_count = state.service.tweet.get_tweets_count(Some(user.id)).await?;
            dao_users.push(DaoUser {
                id: user.id.to_string(),
                username: user.twitter_username.unwrap_or_default(),
                wallet_address: user.wallet_address.unwrap_or_default(),
                score: mining_count as i32,
                role_duration: (chrono::Utc::now() - user.created_at).num_days() as i32,
                activity: mining_count as i32,
                user_icon: "👤".to_string(),
                avatar: None,
            });
        }
    }
    // Apply search filter if provided
    if let Some(search_term) = &query.search {
        let search_lower = search_term.to_lowercase();
        dao_users.retain(|user| {
            user.username.to_lowercase().contains(&search_lower) ||
            user.wallet_address.to_lowercase().contains(&search_lower)
        });
    }
    // Apply sorting if provided
    if let Some(sort_by) = &query.sort_by {
        match sort_by.as_str() {
            "score" => dao_users.sort_by(|a, b| b.score.cmp(&a.score)),
            "activity" => dao_users.sort_by(|a, b| b.activity.cmp(&a.activity)),
            "address" => dao_users.sort_by(|a, b| a.wallet_address.cmp(&b.wallet_address)),
            "roleDuration" => dao_users.sort_by(|a, b| b.role_duration.cmp(&a.role_duration)),
            _ => {} // Default sorting by score
        }
    } else {
        // Default sort by score descending
        dao_users.sort_by(|a, b| b.score.cmp(&a.score));
    }
    Ok(Json(dao_users))
}

pub async fn get_dao_user_count(
    State(state): State<AppState>,
    Extension(_user): Extension<User>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Get user IDs who have at least one reward (i.e., started mining)
    let mining_user_ids = state.service.reward.get_distinct_user_ids_with_rewards().await?;
    let mut mining_users = Vec::new();
    for user_id in mining_user_ids {
        if let Some(user) = state.service.user.get_user_by_id(&user_id).await? {
            let mining_count = state.service.tweet.get_tweets_count(Some(user.id)).await?;
            mining_users.push(json!({
                "id": user.id,
                "username": user.twitter_username,
                "wallet_address": user.wallet_address,
                "mining_count": mining_count
            }));
        }
    }
    Ok(Json(json!({
        "total_users": mining_users.len(),
        "users": mining_users
    })))
}

/// Get enhanced mining status with detailed tweet information
pub async fn get_tweet_mining_status(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<TweetMiningStatusResponse>, ApiError> {
    let total_tweets = state.service.tweet.get_tweets_count(Some(user.id)).await?;
    let phase1_count = state.service.tweet.get_phase1_mining_count(&user.id).await?;
    let phase2_count = state.service.tweet.get_phase2_mining_count(&user.id).await?;
    
    // Get available rewards count (pending rewards)
    let available_rewards = state.service.reward.get_rewards(&Some(user.id), Some(0), Some(1000), Some(true)).await?;
    let pending_rewards = available_rewards.len() as i64;
    
    // Get total claimed rewards
    let claimed_rewards = state.service.reward.get_rewards(&Some(user.id), Some(0), Some(1000), Some(false)).await?;
    let total_rewards_claimed = claimed_rewards.len() as i64;

    let phase1_mining_count_all = state.service.tweet.get_all_phase1_mining_count().await?;
    let mining_phase = get_current_mining_phase(phase1_mining_count_all);
    
    let current_phase = if mining_phase == MiningPhase::Phase2 { 2 } else { 1 };
    
    // Get accumulated rewards from user table
    let accumulated_rewards = user.accumulated_reward.unwrap_or(0);
    
    let response = TweetMiningStatusResponse {
        current_phase,
        total_tweets,
        phase1_count,
        phase2_count,
        pending_rewards,
        total_rewards_claimed,
        accumulated_rewards,
    };
    
    Ok(Json(response))
}

/// Claim reward for a specific tweet (off-chain accumulation)
pub async fn claim_tweet_reward_tx(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    ValidatedRequest(payload): ValidatedRequest<types::dto::ClaimTweetRewardRequest>,
) -> Result<Json<Value>, ApiError> {
    // Claim the reward for this tweet (marks it as claimed in database)
    let claimed_reward = state.service.reward.claim_tweet_reward(&user.id, &payload.tweet_id).await?;
    
    match claimed_reward {
        Some(reward) => {
            // Add the reward amount to user's accumulated_reward field
            let reward_amount = reward.reward_amount.max(0);
            let updated_user = state.service.user.add_accumulated_reward(&user.id, reward_amount).await?;
            
            // Get the updated accumulated reward from the user record
            let accumulated_rewards = updated_user.accumulated_reward.unwrap_or(0);

            Ok(Json(json!({ 
                "message": "Tweet reward claimed successfully and added to your accumulated rewards",
                "reward_amount": reward_amount,
                "accumulated_rewards": accumulated_rewards
            })))
        },
        None => {
            // No reward found for this tweet
            Err(ApiError::BadRequest("No available reward found for this tweet, or reward already claimed".to_string()))
        }
    }
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
        &[USER_CLAIM_SEED, wallet.as_ref()],
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
    let mut transaction = Transaction::new_unsigned(message);
    transaction.message.recent_blockhash = _latest_blockhash;
    
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
        &[USER_CLAIM_SEED, wallet.as_ref()],
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
        &[USER_CLAIM_SEED, wallet.as_ref()],
        &state.program.id(),
    );
    let user_token_ata = spl_associated_token_account::get_associated_token_address(&wallet, &mint);
    
    // Derive claim receipt PDA
    let tweet_id_for_receipt = payload.tweet_id.clone().unwrap_or_else(|| format!("general_claim_{}", Uuid::new_v4()));
    let (claim_receipt, _) = Pubkey::find_program_address(
        &[b"claim_receipt", wallet.as_ref(), tweet_id_for_receipt.as_bytes()],
        &state.program.id(),
    );

    // Check if reward pool PDA exists
    match state.program.rpc().get_account(&reward_pool) {
        Ok(_) => {},
        Err(e) => {
            log::error!("Reward pool PDA not initialized: {:?}", e);
            return Err(ApiError::BadRequest("Reward pool must be initialized before claiming tokens. Please initialize the reward pool first.".into()));
        }
    }

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::ClaimTokensWithRole {
            user: wallet,
            user_claim,
            claim_receipt,
            user_token_ata,
            reward_pool_pda: reward_pool,
            treasury_token_account: treasury,
            mint,
            token_program: spl_token::ID,
            associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
            system_program: system_program::ID,
        })
        .args(snake_contract::instruction::ClaimTokensWithRole {
            amount: payload.amount,
            role,
            tweet_id: tweet_id_for_receipt,
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
    
    // ✅ Use consistent seed derivation across all functions
    let (user_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_ref()], // Keep using as_ref() consistently
        &state.program.id(),
    );
    
    let user_token_ata = spl_associated_token_account::get_associated_token_address(&wallet, &mint);
    let (reward_pool, _) = Pubkey::find_program_address(&[REWARD_POOL_SEED], &state.program.id());
    let treasury_token_account = spl_associated_token_account::get_associated_token_address(&reward_pool, &mint);

    // Derive missing PDAs
    let (global_staking_stats, _) = Pubkey::find_program_address(&[b"global_staking_stats"], &state.program.id());
    let (user_staking_history, _) = Pubkey::find_program_address(&[b"user_staking_history", wallet.as_ref()], &state.program.id());
    
    let mut instructions = Vec::new();
    
    // ✅ Check if user_claim account exists and add initialization if needed
    match state.program.rpc().get_account(&user_claim) {
        Ok(_) => {
            log::info!("UserClaim PDA already exists");
        }
        Err(e) => {
            log::warn!("UserClaim PDA not found, will initialize: {:?}", e);
            let init_claim_ix = state
                .program
                .request()
                .accounts(snake_contract::accounts::InitializeUserClaim {
                    user: wallet,
                    user_claim,
                    system_program: system_program::ID,
                })
                .args(snake_contract::instruction::InitializeUserClaim)
                .instructions()
                .map_err(|e| {
                    log::error!("InitUserClaim build error: {:?}", e);
                    ApiError::InternalServerError("Failed to build InitializeUserClaim".into())
                })?;
            instructions.extend(init_claim_ix);
        }
    }
    
    // ✅ Check if treasury account exists and add creation if needed
    match state.program.rpc().get_account(&treasury_token_account) {
        Ok(_) => {
            log::info!("Treasury token account already exists");
        }
        Err(e) => {
            log::warn!("Treasury token account not found, will create: {:?}", e);
            let create_treasury_ix = spl_associated_token_account::instruction::create_associated_token_account(
                &wallet,
                &reward_pool,
                &mint,
                &spl_token::ID,
            );
            instructions.push(create_treasury_ix);
        }
    }
    
    // ✅ Validate input parameters
    if payload.amount < 5000 {
        return Err(ApiError::BadRequest("Minimum staking amount is 5000 tokens".to_string()));
    }
    
    if payload.duration_months != 3 && payload.duration_months != 6 {
        return Err(ApiError::BadRequest("Duration must be 3 or 6 months".to_string()));
    }
    
    // ✅ Build the lock tokens instruction with correct account names
    let lock_ixs = state
        .program
        .request()
        .accounts(snake_contract::accounts::LockTokens {
            user: wallet,
            user_claim,
            user_token_account: user_token_ata,
            reward_pool_pda: reward_pool,
            treasury_token_account, // Match smart contract account name
            global_staking_stats,
            user_staking_history,
            system_program: system_program::ID,
            token_program: spl_token::ID,
        })
        .args(snake_contract::instruction::LockTokens {
            amount: payload.amount * LAMPORTS_PER_SNK, // Convert to 9 decimals
            duration_months: payload.duration_months,
        })
        .instructions()
        .map_err(|e| {
            log::error!("LockTokens build error: {:?}", e);
            ApiError::InternalServerError("Failed to build LockTokens instruction".into())
        })?;
    
    instructions.extend(lock_ixs);
    
    // ✅ Get latest blockhash
    let latest_blockhash = state
        .program
        .rpc()
        .get_latest_blockhash()
        .map_err(|e| {
            log::error!("Blockhash error: {:?}", e);
            ApiError::InternalServerError("Could not fetch blockhash".into())
        })?;
    
    // ✅ Build unsigned transaction for user to sign
    let message = Message::new_with_blockhash(&instructions, Some(&wallet), &latest_blockhash);
    let transaction = Transaction::new_unsigned(message);
    
    let serialized_transaction = bincode::serialize(&transaction).map_err(|e| {
        log::error!("Serialization error: {:?}", e);
        ApiError::InternalServerError("Failed to serialize transaction".into())
    })?;
    
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
    
    // ✅ Use consistent seed derivation - changed from as_array() to as_ref()
    let (user_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_ref()], // Fixed: use as_ref() like lock_tokens_tx
        &state.program.id(),
    );
    
    let user_token_ata = spl_associated_token_account::get_associated_token_address(&wallet, &mint);
    let (reward_pool, _) = Pubkey::find_program_address(&[REWARD_POOL_SEED], &state.program.id());
    let treasury_token_account = spl_associated_token_account::get_associated_token_address(&reward_pool, &mint);
    let (global_staking_stats, _) = Pubkey::find_program_address(&[b"global_staking_stats"], &state.program.id());
    let (user_staking_history, _) = Pubkey::find_program_address(&[b"user_staking_history", wallet.as_ref()], &state.program.id());

    // ✅ Build unlock instruction with correct account names
    let instructions = state
        .program
        .request()
        .accounts(snake_contract::accounts::UnlockTokens {
            user: wallet,
            user_claim,
            user_token_account: user_token_ata,
            reward_pool_pda: reward_pool,
            treasury_token_account, // Match smart contract account name
            global_staking_stats,
            user_staking_history,
            system_program: system_program::ID,
            token_program: spl_token::ID,
        })
        .args(snake_contract::instruction::UnlockTokens {})
        .instructions()
        .map_err(|e| {
            log::error!("UnlockTokens build error: {:?}", e);
            ApiError::InternalServerError("Failed to build UnlockTokens instruction".into())
        })?;

    // ✅ Improved error handling
    let latest_blockhash = state
        .program
        .rpc()
        .get_latest_blockhash()
        .map_err(|e| {
            log::error!("Blockhash error: {:?}", e);
            ApiError::InternalServerError("Could not fetch blockhash".into())
        })?;

    let message = Message::new_with_blockhash(&instructions, Some(&wallet), &latest_blockhash);
    let transaction = Transaction::new_unsigned(message);
    
    let serialized_transaction = bincode::serialize(&transaction).map_err(|e| {
        log::error!("Serialization error: {:?}", e);
        ApiError::InternalServerError("Failed to serialize transaction".into())
    })?;
    
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
    let treasury = spl_associated_token_account::get_associated_token_address(&reward_pool, &mint);
    
    // ✅ Use consistent seed derivation - changed from as_array() to as_ref()
    let (user_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_ref()], // Fixed: use as_ref() like lock_tokens_tx
        &state.program.id(),
    );
    
    let user_token_ata = spl_associated_token_account::get_associated_token_address(&wallet, &mint);
    let (global_staking_stats, _) = Pubkey::find_program_address(&[b"global_staking_stats"], &state.program.id());
    let (user_staking_history, _) = Pubkey::find_program_address(&[b"user_staking_history", wallet.as_ref()], &state.program.id());

    // ✅ Build claim yield instruction with correct account names
    let instructions = state
        .program
        .request()
        .accounts(snake_contract::accounts::ClaimYield {
            user: wallet,
            user_claim,
            user_token_account: user_token_ata,
            mint,
            reward_pool_pda: reward_pool,
            treasury, // This account name is correct for ClaimYield
            global_staking_stats,
            user_staking_history,
            system_program: system_program::ID,
            token_program: spl_token::ID,
        })
        .args(snake_contract::instruction::ClaimYield {})
        .instructions()
        .map_err(|e| {
            log::error!("ClaimYield build error: {:?}", e);
            ApiError::InternalServerError("Failed to build ClaimYield instruction".into())
        })?;

    // ✅ Improved error handling
    let latest_blockhash = state
        .program
        .rpc()
        .get_latest_blockhash()
        .map_err(|e| {
            log::error!("Blockhash error: {:?}", e);
            ApiError::InternalServerError("Could not fetch blockhash".into())
        })?;

    let message = Message::new_with_blockhash(&instructions, Some(&wallet), &latest_blockhash);
    let transaction = Transaction::new_unsigned(message);
    
    let serialized_transaction = bincode::serialize(&transaction).map_err(|e| {
        log::error!("Serialization error: {:?}", e);
        ApiError::InternalServerError("Failed to serialize transaction".into())
    })?;
    
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
        &[USER_CLAIM_SEED, wallet.as_ref()],
        &state.program.id(),
    );
    let (vesting_account, _) = Pubkey::find_program_address(
        &[VESTING_SEED, wallet.as_ref()],
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

// ========== TCE (Token Claim Event) ENDPOINTS ==========

/// Get TCE status and user's accumulated rewards
pub async fn get_tce_status(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    // Get TCE status from database (faster and no gas fees)
    let tce_started = state.service.values.get_tce_status().await?;

    // Get user's off-chain accumulated rewards from user table
    let off_chain_accumulated = user.accumulated_reward.unwrap_or(0) as u64;

    // Get user's on-chain accumulated rewards if wallet is set (only if needed)
    let on_chain_accumulated = match user.wallet() {
        Some(wallet) => {
            let (user_claim_pda, _) = Pubkey::find_program_address(
                &[USER_CLAIM_SEED, wallet.as_ref()],
                &state.program.id(),
            );

            match state.program.rpc().get_account_data(&user_claim_pda) {
                Ok(data) => {
                    match snake_contract::state::UserClaim::try_deserialize(&mut data.as_slice()) {
                        Ok(user_claim_data) => user_claim_data.accumulated_rewards,
                        Err(_) => 0,
                    }
                },
                Err(_) => 0,
            }
        },
        None => 0,
    };

    Ok(Json(json!({
        "tce_started": tce_started,
        "off_chain_accumulated_rewards": off_chain_accumulated,
        "on_chain_accumulated_rewards": on_chain_accumulated,
        "total_claimable": if tce_started { on_chain_accumulated } else { 0 },
        "wallet_address": user.wallet_address,
        "message": if tce_started {
            "TCE has started! You can now claim your accumulated rewards on-chain."
        } else {
            "TCE has not started yet. Your rewards are being accumulated off-chain."
        }
    })))
}

/// Start TCE (Admin only)
pub async fn start_tce_tx(
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let admin = Arc::new(Keypair::from_base58_string(&state.env.backend_wallet_private_key));

    let (reward_pool_pda, _) = Pubkey::find_program_address(
        &[REWARD_POOL_SEED],
        &state.program.id(),
    );

    let instructions = state
        .program
        .request()
        .accounts(snake_contract::accounts::StartTce {
            reward_pool: reward_pool_pda,
            admin: admin.pubkey(),
            system_program: system_program::ID,
        })
        .args(snake_contract::instruction::StartTce {})
        .instructions()
        .map_err(|e| {
            log::error!("StartTce build error: {:?}", e);
            ApiError::InternalServerError("Failed to build StartTce instruction".into())
        })?;

    let latest_blockhash = state
        .program
        .rpc()
        .get_latest_blockhash()
        .map_err(|e| {
            log::error!("Blockhash error: {:?}", e);
            ApiError::InternalServerError("Could not fetch blockhash".into())
        })?;

    let message = Message::new(&instructions, Some(&admin.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.partial_sign(&[&admin], latest_blockhash);

    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

/// Update TCE status in database after transaction confirmation
pub async fn update_tce_status(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<Value>, ApiError> {
    // Extract the started status from payload
    let started = payload.get("started")
        .and_then(|v| v.as_bool())
        .ok_or_else(|| ApiError::BadRequest("Missing or invalid 'started' field".to_string()))?;

    // Update TCE status in database
    state.service.values.set_tce_status(started).await?;

    Ok(Json(json!({
        "message": format!("TCE status updated to: {}", started),
        "tce_started": started
    })))
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
        &[USER_CLAIM_SEED, wallet.as_ref()],
        &state.program.id(),
    );
    let (vesting_account, _) = Pubkey::find_program_address(
        &[VESTING_SEED, wallet.as_ref()],
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


/// Update accumulated rewards for a specific user (Admin only)
pub async fn update_user_accumulated_rewards_tx(
    Path(user_id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<String>, ApiError> {
    let admin = Arc::new(Keypair::from_base58_string(&state.env.backend_wallet_private_key));
    
    // Parse user_id to UUID and get user
    let user_uuid = Uuid::parse_str(&user_id)
        .map_err(|_| ApiError::BadRequest("Invalid user ID format".to_string()))?;
    
    let user = state.service.user.get_user_by_id(&user_uuid).await?
        .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;
    
    let wallet_address = user.wallet_address
        .ok_or_else(|| ApiError::BadRequest("User has no wallet address".to_string()))?;
    
    let wallet = Pubkey::from_str(&wallet_address)
        .map_err(|_| ApiError::BadRequest("Invalid wallet address".to_string()))?;
    
    // Get amount from payload
    let amount = payload.get("amount")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| ApiError::BadRequest("Missing or invalid amount".to_string()))?;

    let (reward_pool_pda, _) = Pubkey::find_program_address(
        &[REWARD_POOL_SEED],
        &state.program.id(),
    );

    let (user_claim_pda, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_ref()],
        &state.program.id(),
    );

    let instructions = state
        .program
        .request()
        .accounts(snake_contract::accounts::UpdateAccumulatedRewards {
            reward_pool: reward_pool_pda,
            user_claim: user_claim_pda,
            user: wallet,
            admin: admin.pubkey(),
            system_program: system_program::ID,
        })
        .args(snake_contract::instruction::UpdateAccumulatedRewards { amount })
        .instructions()
        .map_err(|e| {
            log::error!("UpdateAccumulatedRewards build error: {:?}", e);
            ApiError::InternalServerError("Failed to build UpdateAccumulatedRewards instruction".into())
        })?;

    let latest_blockhash = state
        .program
        .rpc()
        .get_latest_blockhash()
        .map_err(|e| {
            log::error!("Blockhash error: {:?}", e);
            ApiError::InternalServerError("Could not fetch blockhash".into())
        })?;

    let message = Message::new(&instructions, Some(&admin.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.partial_sign(&[&admin], latest_blockhash);

    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

/// Sync off-chain rewards to on-chain for the authenticated user
/// User must pay transaction fees for this operation
pub async fn sync_rewards_to_chain(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(payload): Json<SyncRewardsRequest>,
) -> Result<Json<Value>, ApiError> {
    // 1. Get user's accumulated off-chain rewards
    let rewards = state.service.get_user_rewards(&user.id, None).await
        .map_err(|e| {
            log::error!("Failed to get user rewards: {:?}", e);
            ApiError::InternalServerError("Failed to get user rewards".into())
        })?;
    
    let total_rewards: u64 = rewards.iter().map(|r| r.reward_amount as u64).sum();
    
    if total_rewards == 0 {
        return Ok(Json(json!({
            "error": "No rewards to sync",
            "pending_rewards": 0
        })));
    }
    
    // 2. Verify user signature (basic validation)
    if payload.user_signature.is_empty() {
        return Err(ApiError::BadRequest("User signature required".into()));
    }
    
    // 3. Create sync transaction that user will pay for
    // This would typically create a blockchain transaction that:
    // - Updates the user's on-chain accumulated rewards
    // - Marks off-chain rewards as synced
    // - User pays the transaction fee
    
    let wallet_address = user.wallet_address.as_ref()
        .ok_or_else(|| ApiError::BadRequest("Wallet address not set".into()))?;
    
    // In a real implementation, you would:
    // - Create a blockchain transaction
    // - Return the transaction for user to sign and submit
    // - Update database to mark rewards as "pending sync"
    
    Ok(Json(json!({
        "message": "Reward sync transaction prepared",
        "pending_rewards": total_rewards,
        "wallet_address": wallet_address,
        "transaction_fee_payer": "user",
        "status": "ready_to_sync",
        "instructions": "User must sign and submit the transaction to complete sync"
    })))
}

#[derive(serde::Deserialize)]
pub struct SyncRewardsRequest {
    pub user_signature: String,
}

/// Get pending (unsynced) rewards for the authenticated user
pub async fn get_pending_rewards(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
) -> Result<Json<Value>, ApiError> {
    // Get user's accumulated off-chain rewards
    let rewards = state.service.get_user_rewards(&user.id, None).await
        .map_err(|e| {
            log::error!("Failed to get user rewards: {:?}", e);
            ApiError::InternalServerError("Failed to get user rewards".into())
        })?;
    
    let total_rewards: u64 = rewards.iter().map(|r| r.reward_amount as u64).sum();
    
    // In a real implementation, you would also check:
    // - Which rewards have already been synced to on-chain
    // - Only return unsynced rewards
    
    Ok(Json(json!({
        "pending_rewards": total_rewards,
        "total_reward_entries": rewards.len(),
        "can_sync": total_rewards > 0,
        "wallet_address": user.wallet_address,
        "sync_required_before_claim": true
    })))
}
