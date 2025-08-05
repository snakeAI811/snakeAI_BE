use crate::state::AppState;
use anchor_client::solana_sdk::{
    message::Message, pubkey::Pubkey, system_program,
    transaction::Transaction,
};
use axum::{Extension, Json, extract::State};
use base64::{Engine, engine};
use types::{
    error::ApiError,
    model::User,
};

const USER_CLAIM_SEED: &[u8] = b"user_claim";

async fn generate_transaction(
    state: &AppState,
    instructions: Vec<anchor_client::solana_sdk::instruction::Instruction>,
    payer: Pubkey,
) -> Result<Json<String>, ApiError> {
    let _latest_blockhash = match state.program.rpc().get_latest_blockhash() {
        Ok(latest_blockhash) => latest_blockhash,
        Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
    };

    let message = Message::new(&instructions, Some(&payer));
    let transaction = Transaction::new_unsigned(message);
    let serialized_transaction = bincode::serialize(&transaction).unwrap();
    let base64_transaction = engine::general_purpose::STANDARD.encode(&serialized_transaction);

    Ok(Json(base64_transaction))
}

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
