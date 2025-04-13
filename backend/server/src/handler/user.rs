use std::str::FromStr;

use crate::state::AppState;
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, message::Message, pubkey::Pubkey, signature::Keypair,
        signer::Signer, transaction::Transaction,
    },
    Client, Cluster,
};
use axum::{
    extract::{Query, State},
    Extension, Json,
};
use base64::{engine, Engine};
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
const REWARD_POOL_SEED: &[u8] = b"reward_pool";
const USER_CLAIM_SEED: &[u8] = b"user_claim";

pub async fn get_claim_tx(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<Json<String>, ApiError> {
    let available_reward = state.service.reward.get_available_reward(&user.id).await?;

    if let Some(available_reward) = available_reward {
        if let Some(wallet) = user.wallet() {
            let admin = Keypair::from_base58_string(&state.env.backend_wallet_private_key);
            let client =
                Client::new_with_options(Cluster::Devnet, &admin, CommitmentConfig::processed());
            let program = client.program(snake_contract::ID).unwrap();

            let mint = Pubkey::from_str("E1BHSRCrWvBe1hVBKjHvUbaA8H2QGWttQva14xr2DEJJ").unwrap();
            let (reward_pool, _) = Pubkey::find_program_address(&[REWARD_POOL_SEED], &program.id());
            let treasury = reward_pool;
            let (user_claim, _) =
                Pubkey::find_program_address(&[USER_CLAIM_SEED, wallet.as_array()], &program.id());
            let user_token_ata = wallet;

            let associated_token_program =
                Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap();
            let token_program = associated_token_program;
            let system_program = associated_token_program;

            let instructions = match program
                .request()
                .accounts(snake_contract::accounts::ClaimReward {
                    user: wallet,
                    admin: admin.pubkey(),
                    reward_pool,
                    treasury,
                    user_claim,
                    user_token_ata,
                    mint,
                    associated_token_program,
                    token_program,
                    system_program,
                })
                .args(snake_contract::instruction::ClaimReward)
                .instructions()
            {
                Ok(ixs) => ixs,
                Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
            };

            let latest_blockhash = match program.rpc().get_latest_blockhash() {
                Ok(latest_blockhash) => latest_blockhash,
                Err(err) => return Err(ApiError::InternalServerError(err.to_string())),
            };

            let message = Message::new(&instructions, Some(&wallet));
            let mut transaction = Transaction::new_unsigned(message);
            transaction.partial_sign(&[&admin], latest_blockhash);
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
