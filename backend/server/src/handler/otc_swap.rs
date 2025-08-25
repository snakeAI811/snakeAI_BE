use crate::state::AppState;
use anchor_client::{
    anchor_lang::{InstructionData, ToAccountMetas},
    solana_sdk::{
        instruction::Instruction, message::Message, pubkey::Pubkey, signature::Keypair,
        signature::Signature, signer::Signer, system_program, transaction::Transaction,
    },
};
use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use base64::{engine, Engine};
use std::str::FromStr;

use snake_contract::constants::{
    LAMPORTS_PER_SNK, OTC_SWAP_SEED, REWARD_POOL_SEED, USER_CLAIM_SEED,
};
use snake_contract::state::SwapType;
use types::{
    dto::{
        AcceptOtcSwapRequest, InitiateOtcSwapEnhancedRequest, InitiateOtcSwapRequest,
        OtcSwapResponse, UpdateOtcSwapTxRequest,
    },
    error::{ApiError, ValidatedRequest},
    model::User,
};

// Get active OTC swaps
pub async fn get_active_swaps(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<types::dto::ActiveSwapsResponse>, ApiError> {
    let page = params
        .get("page")
        .and_then(|p| p.parse::<i32>().ok())
        .unwrap_or(1);
    let per_page = params
        .get("per_page")
        .and_then(|p| p.parse::<i32>().ok())
        .unwrap_or(20)
        .min(100); // Cap at 100 per page

    let user_role = user.role.as_deref().unwrap_or("none");

    match state
        .service
        .otc_swap
        .get_active_swaps(user_role, page, per_page)
        .await
    {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(ApiError::InternalServerError(err.to_string())),
    }
}

// Get user's OTC swaps
pub async fn get_my_swaps(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<types::dto::MySwapsResponse>, ApiError> {
    match state.service.otc_swap.get_user_swaps(&user).await {
        Ok(response) => Ok(Json(response)),
        Err(err) => Err(ApiError::InternalServerError(err.to_string())),
    }
}

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

    match state
        .service
        .otc_swap
        .get_swap_by_pda(&pda, user_role)
        .await
    {
        Ok(swap) => Ok(Json(swap)),
        Err(err) => Err(ApiError::InternalServerError(err.to_string())),
    }
}

/// Initiate OTC swap
pub async fn initiate_otc_swap_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<InitiateOtcSwapRequest>,
) -> Result<Json<String>, ApiError> {
    let wallet = user
        .wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;
    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    let seller_token_ata =
        spl_associated_token_account::get_associated_token_address(&wallet, &mint);
    let (user_claim, _) =
        Pubkey::find_program_address(&[USER_CLAIM_SEED, wallet.as_ref()], &state.program.id());
    // Derive otc_swap PDA using only the expected seeds
    let (otc_swap, _) =
        Pubkey::find_program_address(&[OTC_SWAP_SEED, wallet.as_ref()], &state.program.id());

    // Validate payload
    if payload.token_amount == 0 {
        return Err(ApiError::BadRequest(
            "Token amount must be greater than 0".to_string(),
        ));
    }
    if payload.sol_rate == 0 {
        return Err(ApiError::BadRequest(
            "SOL rate must be greater than 0".to_string(),
        ));
    }

    // Convert role string to enum
    let _buyer_role_required = match payload.buyer_role_required.as_str() {
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
            token_amount: payload.token_amount,
            sol_rate: payload.sol_rate,
            buyer_rebate: payload.buyer_rebate,
            swap_type: SwapType::ExiterToPatron,
        })
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&wallet));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.message.recent_blockhash = latest_blockhash;

    // Check if user already has an active swap and cancel it first
    let wallet_str = user
        .wallet_address
        .as_ref()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    // Force cancel any existing active swaps in database first
    match state
        .service
        .otc_swap
        .force_cancel_swap_by_wallet(wallet_str)
        .await
    {
        Ok(Some(_)) => {
            log::debug!("Auto-cancelled existing swap for wallet: {}", wallet_str);
        }
        Ok(None) => {
            // No existing swap to cancel
        }
        Err(err) => {
            log::debug!("Warning: Failed to auto-cancel existing swap: {}", err);
        }
    }

    // Create database record (will be updated with tx signature after user signs)
    let otc_swap_pda = otc_swap.to_string();
    match state
        .service
        .otc_swap
        .create_swap(&user, &payload, otc_swap_pda, None)
        .await
    {
        Ok(_) => {
            // Successfully created database record
        }
        Err(err) => {
            // Log error but don't fail the transaction creation
            log::debug!("Failed to create OTC swap database record: {}", err);
        }
    }

    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);
    Ok(Json(base64_transaction))
}

/// Initiate Enhanced OTC swap — Phase 1: Build and return transaction
pub async fn initiate_otc_swap_enhanced_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<InitiateOtcSwapEnhancedRequest>,
) -> Result<Json<String>, ApiError> {
    let wallet = user
        .wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    let seller_token_ata =
        spl_associated_token_account::get_associated_token_address(&wallet, &mint);

    let (seller_claim, _) =
        Pubkey::find_program_address(&[USER_CLAIM_SEED, wallet.as_ref()], &state.program.id());
    let (otc_swap, _) =
        Pubkey::find_program_address(&[OTC_SWAP_SEED, wallet.as_ref()], &state.program.id());

    if payload.token_amount == 0 || payload.sol_rate == 0 {
        return Err(ApiError::BadRequest(
            "Invalid token amount or SOL rate".to_string(),
        ));
    }

    let swap_type = match payload.swap_type.as_str() {
        "ExiterToPatron" => SwapType::ExiterToPatron,
        "ExiterToTreasury" => SwapType::ExiterToTreasury,
        "PatronToPatron" => SwapType::PatronToPatron,
        _ => return Err(ApiError::BadRequest("Invalid swap type".to_string())),
    };

    let token_amount_lamports = payload
        .token_amount
        .checked_mul(LAMPORTS_PER_SNK)
        .ok_or_else(|| ApiError::BadRequest("Token amount overflow".to_string()))?;

    let instruction = snake_contract::instruction::InitiateOtcSwap {
        token_amount: token_amount_lamports,
        sol_rate: payload.sol_rate,
        buyer_rebate: payload.buyer_rebate,
        swap_type,
    };

    let accounts = snake_contract::accounts::InitiateOtcSwap {
        seller: wallet,
        seller_claim,
        otc_swap,
        seller_token_account: seller_token_ata,
        token_program: spl_token::id(),
        system_program: system_program::id(),
    };

    let swap_instruction = Instruction {
        program_id: state.program.id(),
        accounts: accounts.to_account_metas(None),
        data: instruction.data(),
    };

    let latest_blockhash =
        state.program.rpc().get_latest_blockhash().map_err(|e| {
            ApiError::InternalServerError(format!("Failed to get blockhash: {}", e))
        })?;

    let message =
        Message::new_with_blockhash(&[swap_instruction], Some(&wallet), &latest_blockhash);
    let transaction = Transaction::new_unsigned(message);

    let serialized = bincode::serialize(&transaction)
        .map_err(|e| ApiError::InternalServerError(format!("Failed to serialize tx: {}", e)))?;

    let encoded = engine::general_purpose::STANDARD.encode(&serialized);
    Ok(Json(encoded))
}

/// Accept OTC swap - FIXED VERSION
pub async fn accept_otc_swap_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<AcceptOtcSwapRequest>,
) -> Result<Json<String>, ApiError> {
    let wallet = user
        .wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    let admin = Keypair::from_base58_string(&state.env.backend_wallet_private_key);
    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    let seller_pubkey = Pubkey::from_str(&payload.seller_pubkey)
        .map_err(|_| ApiError::BadRequest("Invalid seller pubkey".to_string()))?;

    // Check if the swap exists and can be accepted by this user
    match state
        .service
        .otc_swap
        .accept_swap(&user, &payload.seller_pubkey, None)
        .await
    {
        Ok(_) => {
            // Successfully validated and prepared database update
        }
        Err(err) => {
            return Err(ApiError::BadRequest(format!("Cannot accept swap: {}", err)));
        }
    }

    let buyer_token_ata =
        spl_associated_token_account::get_associated_token_address(&wallet, &mint);
    let seller_token_ata =
        spl_associated_token_account::get_associated_token_address(&seller_pubkey, &mint);

    // Fixed: Use consistent seeds for PDAs
    let (buyer_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_ref()], // Use USER_CLAIM_SEED constant
        &state.program.id(),
    );
    let (otc_swap, _) = Pubkey::find_program_address(
        &[OTC_SWAP_SEED, seller_pubkey.as_ref()],
        &state.program.id(),
    );

    let (reward_pool, reward_pool_bump) =
        Pubkey::find_program_address(&[REWARD_POOL_SEED], &state.program.id());
    let treasury = spl_associated_token_account::get_associated_token_address(&reward_pool, &mint);
    let (daily_volume_tracker, _) =
        Pubkey::find_program_address(&[b"daily_volume_tracker"], &state.program.id());

    let (seller_claim, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, seller_pubkey.as_ref()], // Use USER_CLAIM_SEED constant
        &state.program.id(),
    );

    // Pre-validate buyer has sufficient SOL
    let buyer_balance = match state.program.rpc().get_balance(&wallet) {
        Ok(balance) => balance,
        Err(err) => {
            return Err(ApiError::InternalServerError(format!(
                "Failed to get buyer balance: {}",
                err
            )))
        }
    };

    // Get swap details to calculate required SOL
    let swap_account = match state.program.rpc().get_account(&otc_swap) {
        Ok(account) => account,
        Err(_) => {
            return Err(ApiError::BadRequest(
                "Swap not found on blockchain".to_string(),
            ))
        }
    };

    // Basic validation that buyer has some SOL for fees and swap
    if buyer_balance < 1_000_000 {
        // Less than 0.001 SOL
        return Err(ApiError::BadRequest(
            "Insufficient SOL balance for transaction".to_string(),
        ));
    }

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::AcceptOtcSwap {
            buyer: wallet,
            buyer_claim,
            otc_swap,
            seller_claim,
            buyer_token_account: buyer_token_ata,
            seller_token_account: seller_token_ata,
            treasury_account: treasury,
            reward_pool,
            mint,
            daily_volume_tracker,
            token_program: spl_token::ID,
            system_program: system_program::ID,
        })
        .args(snake_contract::instruction::AcceptOtcSwap { buyer_rebate: 0 })
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => {
            return Err(ApiError::InternalServerError(format!(
                "Failed to build instructions: {}",
                err
            )))
        }
    };

    let latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => {
            return Err(ApiError::InternalServerError(format!(
                "Failed to get blockhash: {}",
                err
            )))
        }
    };

    let message = Message::new_with_blockhash(&instructions, Some(&wallet), &latest_blockhash);
    let mut transaction = Transaction::new_unsigned(message);

    // Admin signs first (partial signature)
    transaction.partial_sign(&[&admin], latest_blockhash);

    log::info!("Accept swap transaction created:");
    log::info!("- Buyer: {}", wallet);
    log::info!("- Seller: {}", seller_pubkey);
    log::info!("- Swap PDA: {}", otc_swap);
    log::info!("- Instructions: {}", instructions.len());

    let serialized_transaction = bincode::serialize(&transaction).map_err(|e| {
        ApiError::InternalServerError(format!("Failed to serialize transaction: {}", e))
    })?;
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

// Updated handler to fix the transaction creation issue
pub async fn cancel_otc_swap_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let wallet = user
        .wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    let seller_token_ata =
        spl_associated_token_account::get_associated_token_address(&wallet, &mint);
    let (otc_swap, _) =
        Pubkey::find_program_address(&[OTC_SWAP_SEED, wallet.as_ref()], &state.program.id());

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::CancelOtcSwap {
            seller: wallet,
            otc_swap,
            system_program: system_program::ID,
        })
        .args(snake_contract::instruction::CancelOtcSwap {})
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    // Create unsigned transaction for user to sign
    let message = Message::new(&instructions, Some(&wallet));
    let mut transaction = Transaction::new_unsigned(message);
    transaction.message.recent_blockhash = latest_blockhash;

    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

#[derive(serde::Deserialize)]
pub struct ProcessSignedTxRequest {
    pub signed_transaction: String,
}

pub async fn process_cancel_otc_swap_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(signed_tx_data): Json<ProcessSignedTxRequest>,
) -> Result<Json<OtcSwapResponse>, ApiError> {
    log::info!(
        "🔍 Starting process_cancel_otc_swap_tx for user: {}",
        user.id
    );
    log::info!(
        "📦 Received signed_transaction length: {}",
        signed_tx_data.signed_transaction.len()
    );

    // Step 1: Deserialize the signed transaction
    log::info!("🔧 Step 1: Decoding base64 transaction...");
    let signed_transaction_bytes = engine::general_purpose::STANDARD
        .decode(&signed_tx_data.signed_transaction)
        .map_err(|e| {
            log::error!("❌ Failed to decode base64 transaction: {:?}", e);
            ApiError::BadRequest("Invalid base64 transaction".to_string())
        })?;

    log::info!(
        "✅ Base64 decoded successfully, bytes length: {}",
        signed_transaction_bytes.len()
    );

    log::info!("🔧 Step 2: Deserializing transaction...");
    let signed_transaction: Transaction =
        bincode::deserialize(&signed_transaction_bytes).map_err(|e| {
            log::error!("❌ Failed to deserialize transaction: {:?}", e);
            log::error!(
                "Raw bytes (first 50): {:?}",
                &signed_transaction_bytes[..50.min(signed_transaction_bytes.len())]
            );
            ApiError::BadRequest("Invalid transaction data".to_string())
        })?;

    log::info!("✅ Transaction deserialized successfully");
    log::info!(
        "📝 Transaction details: signers={}, instructions={}",
        signed_transaction.signatures.len(),
        signed_transaction.message.instructions.len()
    );

    // Step 3: Enhanced transaction validation
    log::info!("🔧 Step 3: Validating transaction signature...");
    if !signed_transaction.is_signed() {
        log::error!("❌ Transaction not signed");
        return Err(ApiError::BadRequest("Transaction not signed".to_string()));
    }
    log::info!("✅ Transaction is signed");

    // Step 4: Verify the user is the expected signer
    log::info!("🔧 Step 4: Validating signer...");
    let wallet = user.wallet().ok_or_else(|| {
        log::error!("❌ User wallet not set for user: {}", user.id);
        ApiError::BadRequest("User wallet not set".to_string())
    })?;

    log::info!("👤 Expected wallet: {}", wallet);

    // Check if the user's wallet is in the transaction signers
    let message_signers = signed_transaction
        .message
        .account_keys
        .iter()
        .take(signed_transaction.message.header.num_required_signatures as usize)
        .collect::<Vec<_>>();

    log::info!("📋 Transaction signers: {:?}", message_signers);

    if !message_signers.contains(&&wallet) {
        log::error!(
            "❌ Transaction signer mismatch. Expected: {}, Found signers: {:?}",
            wallet,
            message_signers
        );
        return Err(ApiError::BadRequest(
            "Transaction not signed by the expected wallet".to_string(),
        ));
    }
    log::info!("✅ Signer validation passed");

    // Step 5: Validate that this is actually a cancel OTC swap transaction
    log::info!("🔧 Step 5: Validating instruction...");
    let program_id = state.program.id();
    log::info!("🏷️ Expected program ID: {}", program_id);

    let has_cancel_instruction = signed_transaction
        .message
        .instructions
        .iter()
        .enumerate()
        .any(|(idx, ix)| {
            log::info!(
                "📝 Checking instruction {}: program_id_index={}, data_len={}",
                idx,
                ix.program_id_index,
                ix.data.len()
            );

            if let Some(program_key) = signed_transaction
                .message
                .account_keys
                .get(ix.program_id_index as usize)
            {
                log::info!("🔍 Instruction {} uses program: {}", idx, program_key);
                let matches_program = *program_key == program_id;
                let has_data = ix.data.len() > 0;
                log::info!(
                    "🎯 Program match: {}, Has data: {}",
                    matches_program,
                    has_data
                );
                matches_program && has_data
            } else {
                log::warn!(
                    "⚠️ Invalid program_id_index for instruction {}: {}",
                    idx,
                    ix.program_id_index
                );
                false
            }
        });

    if !has_cancel_instruction {
        log::error!("❌ Transaction does not contain expected cancel instruction");
        return Err(ApiError::BadRequest(
            "Transaction does not contain expected cancel instruction".to_string(),
        ));
    }
    log::info!("✅ Instruction validation passed");

    // Step 6: Check if user actually has an active swap to cancel
    log::info!("🔧 Step 6: Checking for active swaps...");
    let wallet_str = user.wallet_address.as_ref().ok_or_else(|| {
        log::error!("❌ User wallet address not set for user: {}", user.id);
        ApiError::BadRequest("User wallet not set".to_string())
    })?;

    log::info!("🔍 Querying swaps for wallet: {}", wallet_str);
    let user_swaps = state
        .service
        .otc_swap
        .get_all_swaps_for_wallet(wallet_str)
        .await
        .map_err(|e| {
            log::error!("❌ Failed to query user swaps: {:?}", e);
            ApiError::InternalServerError(format!("Failed to query user swaps: {}", e))
        })?;

    log::info!("📊 Found {} total swaps for user", user_swaps.len());

    let active_swaps: Vec<_> = user_swaps
        .iter()
        .filter(|swap_with_users| {
            let is_active = swap_with_users.swap.status == "active";
            let not_expired = !swap_with_users.swap.is_expired();
            log::info!(
                "📋 Swap {}: status={}, expired={}",
                swap_with_users.swap.id,
                swap_with_users.swap.status,
                swap_with_users.swap.is_expired()
            );
            is_active && not_expired
        })
        .collect();

    log::info!("🎯 Found {} active, non-expired swaps", active_swaps.len());

    if active_swaps.is_empty() {
        log::error!("❌ No active swap found to cancel");
        return Err(ApiError::BadRequest(
            "No active swap found to cancel".to_string(),
        ));
    }
    log::info!("✅ Active swap validation passed");

    // Step 7: Submit transaction with retries
    log::info!("🔧 Step 7: Submitting transaction to blockchain...");
    let signature = {
        let mut attempts = 0;
        let max_attempts = 3;

        loop {
            attempts += 1;
            log::info!("🔄 Transaction attempt {} of {}", attempts, max_attempts);

            match state
                .program
                .rpc()
                .send_and_confirm_transaction(&signed_transaction)
            {
                Ok(sig) => {
                    log::info!("✅ Cancel OTC swap transaction successful: {}", sig);
                    break sig;
                }
                Err(err) => {
                    log::error!("❌ Transaction attempt {} failed: {:?}", attempts, err);

                    if attempts >= max_attempts {
                        let error_msg = if err.to_string().contains("already processed") {
                            "Transaction already processed"
                        } else if err.to_string().contains("insufficient funds") {
                            "Insufficient SOL for transaction fees"
                        } else if err.to_string().contains("signature verification failed") {
                            "Invalid transaction signature"
                        } else if err.to_string().contains("blockhash not found") {
                            "Transaction expired (blockhash too old)"
                        } else {
                            "Transaction failed on blockchain"
                        };

                        log::error!("❌ All transaction attempts failed: {}", error_msg);
                        return Err(ApiError::InternalServerError(format!(
                            "{}: {}",
                            error_msg, err
                        )));
                    }

                    log::info!("⏳ Waiting 1s before retry...");
                    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                }
            }
        }
    };

    // Step 8: Verify transaction success
    log::info!("🔧 Step 8: Verifying transaction success...");
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    log::info!("✅ Transaction confirmed with signature: {}", signature);

    // Step 9: Update database
    log::info!("🔧 Step 9: Updating database...");
    match state
        .service
        .otc_swap
        .cancel_swap(&user, Some(signature.to_string()))
        .await
    {
        Ok(response) => {
            log::info!(
                "✅ Successfully cancelled OTC swap for user {} with tx signature {}",
                user.id,
                signature
            );
            Ok(Json(response))
        }
        Err(err) => {
            log::error!("❌ Database update failed after successful blockchain transaction. Signature: {}, Error: {}", 
                       signature, err);

            Err(ApiError::InternalServerError(
                format!("Transaction succeeded on blockchain (signature: {}) but database update failed: {}. Please contact support.", 
                       signature, err)
            ))
        }
    }
}

/// Update OTC swap with transaction signature after user signs the transaction
pub async fn update_otc_swap_tx_signature(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(payload): Json<types::dto::UpdateOtcSwapTxRequest>,
) -> Result<Json<String>, ApiError> {
    let wallet = user
        .wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    // 1. Parse the signature from base58 format
    let signature = Signature::from_str(&payload.txSignature)
        .map_err(|e| ApiError::BadRequest(format!("Invalid signature format: {}", e)))?;

    // 2. Verify the transaction exists and is confirmed on-chain
    let confirmation_status = {
        let mut attempts = 0;
        loop {
            match state.program.rpc().confirm_transaction(&signature) {
                Ok(confirmed) => {
                    if confirmed {
                        break true;
                    } else {
                        // Transaction not yet confirmed, retry
                        attempts += 1;
                        if attempts >= 10 {
                            // Wait longer for confirmation
                            return Err(ApiError::BadRequest(
                                "Transaction not confirmed after retries".to_string(),
                            ));
                        }
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }
                }
                Err(e) => {
                    attempts += 1;
                    if attempts >= 3 {
                        return Err(ApiError::InternalServerError(format!(
                            "Transaction confirmation failed: {}",
                            e
                        )));
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                }
            }
        }
    };

    if !confirmation_status {
        return Err(ApiError::BadRequest(
            "Transaction not confirmed".to_string(),
        ));
    }

    // 3. Derive expected swap PDA
    let (otc_swap_pda, _) =
        Pubkey::find_program_address(&[OTC_SWAP_SEED, wallet.as_ref()], &state.program.id());

    // 4. Ensure swap PDA account now exists
    let account_info = state
        .program
        .rpc()
        .get_account(&otc_swap_pda)
        .map_err(|e| {
            log::error!("Failed to fetch swap account {}: {}", otc_swap_pda, e);
            ApiError::InternalServerError("Swap account not found after transaction".to_string())
        })?;

    if account_info.data.is_empty() {
        return Err(ApiError::InternalServerError(
            "Swap account has no data".to_string(),
        ));
    }

    // 5. Ensure swap PDA account now exists
    let account_info = state
        .program
        .rpc()
        .get_account(&otc_swap_pda)
        .map_err(|e| {
            log::error!("Failed to fetch swap account {}: {}", otc_swap_pda, e);
            ApiError::InternalServerError("Swap account not found after transaction".to_string())
        })?;

    if account_info.data.is_empty() {
        return Err(ApiError::InternalServerError(
            "Swap account has no data".to_string(),
        ));
    }

    // 6. ✅ Save to DB now (it's confirmed and on-chain)
    let db_result = state
        .service
        .otc_swap
        .create_enhanced_swap(
            user.id,
            &wallet.to_string(),
            &otc_swap_pda.to_string(),
            payload.token_amount as i64,
            payload.sol_rate as i64,
            payload.buyer_rebate as i64,
            &payload.swap_type,
        )
        .await;

    match db_result {
        Ok(_) => {
            log::info!(
                "Successfully created OTC swap - User: {}, Wallet: {}, PDA: {}, Signature: {}",
                user.id,
                wallet,
                otc_swap_pda,
                signature
            );
            Ok(Json(signature.to_string()))
        }
        Err(e) => {
            log::error!(
                "DB insert failed after on-chain success for signature {}: {}",
                signature,
                e
            );
            // The transaction succeeded on-chain but DB failed
            // You might want to implement a retry mechanism or manual reconciliation
            Err(ApiError::InternalServerError(format!(
                "Swap created on-chain (sig: {}) but DB insert failed. Please contact support.",
                signature
            )))
        }
    }
}

// Fix 1: Handle potential transaction format issues
pub async fn process_cancel_otc_swap_tx_fixed(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(signed_tx_data): Json<ProcessSignedTxRequest>,
) -> Result<Json<OtcSwapResponse>, ApiError> {
    // Enhanced base64 decoding with better error handling
    let signed_transaction_bytes =
        match engine::general_purpose::STANDARD.decode(&signed_tx_data.signed_transaction) {
            Ok(bytes) => {
                log::info!("Successfully decoded base64, bytes length: {}", bytes.len());
                bytes
            }
            Err(e) => {
                log::error!("Base64 decode error: {:?}", e);
                log::error!(
                    "Input string length: {}, first 100 chars: {}",
                    signed_tx_data.signed_transaction.len(),
                    &signed_tx_data.signed_transaction
                        [..100.min(signed_tx_data.signed_transaction.len())]
                );
                return Err(ApiError::BadRequest(
                    "Invalid base64 transaction encoding".to_string(),
                ));
            }
        };

    // Enhanced transaction deserialization with detailed error info
    let signed_transaction: Transaction = match bincode::deserialize(&signed_transaction_bytes) {
        Ok(tx) => {
            log::info!("Successfully deserialized transaction");
            tx
        }
        Err(e) => {
            log::error!("Bincode deserialization error: {:?}", e);
            log::error!(
                "Bytes length: {}, first 50 bytes: {:?}",
                signed_transaction_bytes.len(),
                &signed_transaction_bytes[..50.min(signed_transaction_bytes.len())]
            );

            // Try to provide more helpful error context
            if signed_transaction_bytes.len() < 64 {
                return Err(ApiError::BadRequest(
                    "Transaction data too short".to_string(),
                ));
            }

            return Err(ApiError::BadRequest(
                "Invalid transaction format - deserialization failed".to_string(),
            ));
        }
    };

    // Fix 2: More robust signer validation
    let wallet = user
        .wallet()
        .ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;

    // Check transaction has signatures
    if signed_transaction.signatures.is_empty() {
        log::error!("Transaction has no signatures");
        return Err(ApiError::BadRequest("Transaction not signed".to_string()));
    }

    // More detailed signer checking
    let required_signatures = signed_transaction.message.header.num_required_signatures as usize;
    let available_signatures = signed_transaction.signatures.len();

    if available_signatures < required_signatures {
        log::error!(
            "Insufficient signatures: need {}, have {}",
            required_signatures,
            available_signatures
        );
        return Err(ApiError::BadRequest(
            "Transaction not fully signed".to_string(),
        ));
    }

    // Check wallet is among the signers (first N account keys are signers)
    let message_signers = signed_transaction
        .message
        .account_keys
        .iter()
        .take(required_signatures)
        .collect::<Vec<_>>();

    if !message_signers.contains(&&wallet) {
        log::error!(
            "Wallet {} not found in signers: {:?}",
            wallet,
            message_signers
        );
        return Err(ApiError::BadRequest(
            "Transaction not signed by user wallet".to_string(),
        ));
    }

    // Fix 3: More flexible instruction validation
    let program_id = state.program.id();
    let has_our_program_instruction = signed_transaction.message.instructions.iter().any(|ix| {
        match signed_transaction
            .message
            .account_keys
            .get(ix.program_id_index as usize)
        {
            Some(program_key) => {
                let matches_program = *program_key == program_id;
                log::info!(
                    "Instruction uses program: {}, matches: {}",
                    program_key,
                    matches_program
                );
                matches_program
            }
            None => {
                log::warn!("Invalid program_id_index: {}", ix.program_id_index);
                false
            }
        }
    });

    if !has_our_program_instruction {
        log::error!("No instruction for our program {} found", program_id);
        // Log all instructions for debugging
        for (i, ix) in signed_transaction.message.instructions.iter().enumerate() {
            if let Some(prog) = signed_transaction
                .message
                .account_keys
                .get(ix.program_id_index as usize)
            {
                log::info!(
                    "Instruction {}: program {}, data len {}",
                    i,
                    prog,
                    ix.data.len()
                );
            }
        }
        return Err(ApiError::BadRequest(
            "Transaction does not contain expected program instruction".to_string(),
        ));
    }

    // Fix 4: Better active swap validation with detailed logging
    let wallet_str = user
        .wallet_address
        .as_ref()
        .ok_or_else(|| ApiError::BadRequest("User wallet address not set".to_string()))?;

    // Add timeout to database query
    let user_swaps = match tokio::time::timeout(
        tokio::time::Duration::from_secs(10),
        state.service.otc_swap.get_all_swaps_for_wallet(wallet_str),
    )
    .await
    {
        Ok(Ok(swaps)) => swaps,
        Ok(Err(db_err)) => {
            log::error!("Database error querying swaps: {:?}", db_err);
            return Err(ApiError::InternalServerError(
                "Database query failed".to_string(),
            ));
        }
        Err(_) => {
            log::error!("Database query timeout");
            return Err(ApiError::InternalServerError(
                "Database query timeout".to_string(),
            ));
        }
    };

    log::info!("Found {} swaps for wallet {}", user_swaps.len(), wallet_str);

    // More detailed swap status logging
    let mut active_count = 0;
    let mut expired_count = 0;
    let mut other_status_count = 0;

    for swap_with_users in &user_swaps {
        let swap = &swap_with_users.swap;
        if swap.status == "active" {
            if swap.is_expired() {
                expired_count += 1;
                log::info!("Swap {} is active but expired", swap.id);
            } else {
                active_count += 1;
                log::info!("Swap {} is active and not expired", swap.id);
            }
        } else {
            other_status_count += 1;
            log::info!("Swap {} has status: {}", swap.id, swap.status);
        }
    }

    log::info!(
        "Swap summary - Active: {}, Expired: {}, Other status: {}",
        active_count,
        expired_count,
        other_status_count
    );

    if active_count == 0 {
        return Err(ApiError::BadRequest(
            "No active non-expired swap found to cancel".to_string(),
        ));
    }

    // Fix 5: Enhanced transaction submission with better error handling
    let signature = match submit_transaction_with_retries(&state, &signed_transaction).await {
        Ok(sig) => sig,
        Err(e) => return Err(e),
    };

    // Fix 6: Database update with better error context
    log::info!(
        "Updating database for successful transaction: {}",
        signature
    );
    match state
        .service
        .otc_swap
        .cancel_swap(&user, Some(signature.to_string()))
        .await
    {
        Ok(response) => {
            log::info!("Successfully cancelled OTC swap for user {}", user.id);
            Ok(Json(response))
        }
        Err(err) => {
            log::error!("Database update failed: {:?}", err);
            // Return more specific error information
            Err(ApiError::InternalServerError(format!(
                "Transaction succeeded (sig: {}) but database update failed. Error: {}",
                signature, err
            )))
        }
    }
}

// Helper function for transaction submission with retries
async fn submit_transaction_with_retries(
    state: &AppState,
    transaction: &Transaction,
) -> Result<Signature, ApiError> {
    let max_attempts = 3;
    let mut last_error = None;

    for attempt in 1..=max_attempts {
        log::info!(
            "Transaction submission attempt {}/{}",
            attempt,
            max_attempts
        );

        match state
            .program
            .rpc()
            .send_and_confirm_transaction(transaction)
        {
            Ok(signature) => {
                log::info!(
                    "Transaction successful on attempt {}: {}",
                    attempt,
                    signature
                );
                return Ok(signature);
            }
            Err(err) => {
                log::error!("Transaction attempt {} failed: {:?}", attempt, err);

                // Check conditions BEFORE moving err
                let should_break = err.to_string().contains("already processed")
                    || err.to_string().contains("signature verification failed")
                    || err.to_string().contains("insufficient funds");

                last_error = Some(err); // Now we can safely move err

                if should_break {
                    break;
                }

                if attempt < max_attempts {
                    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                }
            }
        }
    }

    // All attempts failed
    if let Some(err) = last_error {
        let error_msg = categorize_transaction_error(&err.to_string());
        log::error!("All transaction attempts failed: {}", error_msg);
        Err(ApiError::InternalServerError(format!(
            "{}: {}",
            error_msg, err
        )))
    } else {
        Err(ApiError::InternalServerError(
            "Transaction failed for unknown reason".to_string(),
        ))
    }
}

fn categorize_transaction_error(error_str: &str) -> &'static str {
    if error_str.contains("already processed") {
        "Transaction already processed"
    } else if error_str.contains("insufficient funds") {
        "Insufficient SOL for transaction fees"
    } else if error_str.contains("signature verification failed") {
        "Invalid transaction signature"
    } else if error_str.contains("blockhash not found") {
        "Transaction expired (blockhash too old)"
    } else if error_str.contains("timeout") {
        "Transaction confirmation timeout"
    } else if error_str.contains("network") || error_str.contains("connection") {
        "Network connection error"
    } else {
        "Transaction failed on blockchain"
    }
}
