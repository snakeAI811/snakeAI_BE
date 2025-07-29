use std::str::FromStr;

use crate::state::AppState;
use anchor_client::solana_sdk::{
    message::Message, pubkey::Pubkey, signature::Keypair, signer::Signer, system_program,
    transaction::Transaction,
};
use axum::{Extension, Json, extract::State};
use base64::{Engine, engine};
use serde_json::{json, Value};
use types::{
    dto::{
        SelectRoleRequest, PatronApplicationRequest, ApprovePatronRequest, RevokePatronRequest,
        CreateVestingRequest, LockTokensRequest, ClaimYieldRequest, InitiateOtcSwapRequest,
        ExecuteOtcSwapRequest, CancelOtcSwapRequest,
    },
    error::{ApiError, ValidatedRequest},
    model::User,
};

const USER_CLAIM_SEED: &[u8] = b"user_claim";
const VESTING_SEED: &[u8] = b"vesting";
const ESCROW_SEED: &[u8] = b"escrow";
const OTC_SWAP_SEED: &[u8] = b"otc_swap";

// Initialize User Claim
pub async fn get_initialize_user_claim_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet().ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;
    
    let (user_claim_pda, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::InitializeUserClaim {
            user: wallet,
            user_claim: user_claim_pda,
            system_program: system_program::ID,
        })
        .args(snake_contract::instruction::InitializeUserClaim)
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    generate_transaction(&state, instructions, wallet).await
}

// Role Selection
pub async fn get_select_role_tx(
    Extension(user): Extension<User>,
    ValidatedRequest(payload): ValidatedRequest<SelectRoleRequest>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet().ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;
    
    let (user_claim_pda, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );

    let role = match payload.role.as_str() {
        "staker" => snake_contract::UserRole::Staker {},
        "patron" => snake_contract::UserRole::Patron {},
        _ => return Err(ApiError::BadRequest("Invalid role. Valid roles are: staker, patron".to_string())),
    };

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::SelectRole {
            user: wallet,
            user_claim: user_claim_pda,
        })
        .args(snake_contract::instruction::SelectRole { role })
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    generate_transaction(&state, instructions, wallet).await
}

// Patron Application
pub async fn get_patron_application_tx(
    Extension(user): Extension<User>,
    ValidatedRequest(payload): ValidatedRequest<PatronApplicationRequest>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet().ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;
    
    let (user_claim_pda, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::ApplyForPatron {
            user: wallet,
            user_claim: user_claim_pda,
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

    generate_transaction(&state, instructions, wallet).await
}

// Create Vesting
pub async fn get_create_vesting_tx(
    Extension(user): Extension<User>,
    ValidatedRequest(payload): ValidatedRequest<CreateVestingRequest>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet().ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;
    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    
    let (user_claim_pda, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );
    
    let (vesting_pda, _) = Pubkey::find_program_address(
        &[VESTING_SEED, wallet.as_array()],
        &state.program.id(),
    );
    
    let (escrow_pda, _) = Pubkey::find_program_address(
        &[ESCROW_SEED, wallet.as_array()],
        &state.program.id(),
    );

    let user_token_account = spl_associated_token_account::get_associated_token_address(&wallet, &mint);
    let escrow_token_account = spl_associated_token_account::get_associated_token_address(&escrow_pda, &mint);

    let role_type = match payload.role_type.as_str() {
        "staker" => snake_contract::UserRole::Staker {},
        "patron" => snake_contract::UserRole::Patron {},
        _ => return Err(ApiError::BadRequest("Invalid role type".to_string())),
    };

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::CreateVesting {
            user: wallet,
            user_claim: user_claim_pda,
            vesting_account: vesting_pda,
            user_token_account,
            escrow_token_account,
            token_program: spl_token::ID,
            system_program: system_program::ID,
        })
        .args(snake_contract::instruction::CreateVesting {
            amount: payload.amount,
            role_type,
        })
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    generate_transaction(&state, instructions, wallet).await
}

// Lock Tokens
pub async fn get_lock_tokens_tx(
    Extension(user): Extension<User>,
    ValidatedRequest(payload): ValidatedRequest<LockTokensRequest>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet().ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;
    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    
    let (user_claim_pda, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );

    let user_token_account = spl_associated_token_account::get_associated_token_address(&wallet, &mint);

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::LockTokens {
            user: wallet,
            user_claim: user_claim_pda,
            user_token_account,
            token_mint: mint,
            token_program: spl_token::ID,
            system_program: system_program::ID,
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

    generate_transaction(&state, instructions, wallet).await
}

// Unlock Tokens
pub async fn get_unlock_tokens_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet().ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;
    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    
    let (user_claim_pda, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );

    let user_token_account = spl_associated_token_account::get_associated_token_address(&wallet, &mint);

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::UnlockTokens {
            user: wallet,
            user_claim: user_claim_pda,
            user_token_account,
            token_mint: mint,
            token_program: spl_token::ID,
        })
        .args(snake_contract::instruction::UnlockTokens)
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    generate_transaction(&state, instructions, wallet).await
}

// Claim Yield
pub async fn get_claim_yield_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet().ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;
    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    
    let (user_claim_pda, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );

    let user_token_account = spl_associated_token_account::get_associated_token_address(&wallet, &mint);

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::ClaimYield {
            user: wallet,
            user_claim: user_claim_pda,
            user_token_account,
            token_mint: mint,
            token_program: spl_token::ID,
        })
        .args(snake_contract::instruction::ClaimYield)
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    generate_transaction(&state, instructions, wallet).await
}

// Initiate OTC Swap
pub async fn get_initiate_otc_swap_tx(
    Extension(user): Extension<User>,
    ValidatedRequest(payload): ValidatedRequest<InitiateOtcSwapRequest>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let wallet = user.wallet().ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;
    let mint = Pubkey::from_str(&state.env.token_mint).unwrap();
    
    let (seller_claim_pda, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );
    
    let (otc_swap_pda, _) = Pubkey::find_program_address(
        &[OTC_SWAP_SEED, wallet.as_array()],
        &state.program.id(),
    );

    let seller_token_account = spl_associated_token_account::get_associated_token_address(&wallet, &mint);

    let swap_type = match payload.swap_type.as_str() {
        "NormalToPatron" => snake_contract::SwapType::NormalToPatron {},
        "PatronToNormal" => snake_contract::SwapType::PatronToNormal {},
        "NormalToNormal" => snake_contract::SwapType::NormalToNormal {},
        _ => return Err(ApiError::BadRequest("Invalid swap type".to_string())),
    };

    let instructions = match state
        .program
        .request()
        .accounts(snake_contract::accounts::InitiateOtcSwapEnhanced {
            seller: wallet,
            seller_claim: seller_claim_pda,
            seller_token_account,
            otc_swap: otc_swap_pda,
            token_mint: mint,
            token_program: spl_token::ID,
            system_program: system_program::ID,
        })
        .args(snake_contract::instruction::InitiateOtcSwapEnhanced {
            token_amount: payload.token_amount,
            sol_rate: payload.sol_rate,
            buyer_rebate: payload.buyer_rebate,
            swap_type,
        })
        .instructions()
    {
        Ok(ixs) => ixs,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    generate_transaction(&state, instructions, wallet).await
}

// Get user status (role, vesting, locks, etc.)
pub async fn get_user_status(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<Value>, ApiError> {
    let wallet = user.wallet().ok_or_else(|| ApiError::BadRequest("User wallet not set".to_string()))?;
    
    let (user_claim_pda, _) = Pubkey::find_program_address(
        &[USER_CLAIM_SEED, wallet.as_array()],
        &state.program.id(),
    );

    // Try to fetch user claim account
    let user_claim_data = match state.program.rpc().get_account_data(&user_claim_pda) {
        Ok(data) => {
            // Parse the account data (this would need proper deserialization)
            Some("Account exists")
        },
        Err(_) => None,
    };

    Ok(Json(json!({
        "wallet": wallet.to_string(),
        "user_claim_pda": user_claim_pda.to_string(),
        "user_claim_exists": user_claim_data.is_some(),
        "user_id": user.id
    })))
}

// Helper function to generate transaction
async fn generate_transaction(
    state: &AppState,
    instructions: Vec<anchor_client::solana_sdk::instruction::Instruction>,
    wallet: Pubkey,
) -> Result<Json<String>, ApiError> {
    let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(blockhash) => blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&wallet));
    let transaction = Transaction::new_unsigned(message);
    let mut transaction_with_blockhash = transaction;
    transaction_with_blockhash.message.recent_blockhash = _latest_blockhash;
    
    let serialized_transaction = bincode::serialize(&transaction_with_blockhash).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}
