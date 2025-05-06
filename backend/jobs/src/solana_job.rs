use base64::Engine;
use borsh::BorshDeserialize;
use chrono::{DateTime, Utc};
use database::AppService;
use serde::Serialize;
use solana_client::{
    rpc_client::{GetConfirmedSignaturesForAddress2Config, RpcClient},
    rpc_response::RpcConfirmedTransactionStatusWithSignature,
};
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use solana_transaction_status::{option_serializer::OptionSerializer, UiTransactionEncoding};
use std::{str::FromStr, sync::Arc};
use utils::env::Env;

use crate::twitter_job::TwitterClient;

pub struct SolanaClient {
    client: RpcClient,
    program_id: Pubkey,
}

#[derive(Debug, Serialize, BorshDeserialize)]
pub struct ClaimedReward {
    pub discriminator: [u8; 5],
    pub user: Pubkey,
    pub reward_amount: i64,
    pub burn_amount: i64,
    pub reward_level: u8,
}

pub struct ClaimTx {
    pub reward: ClaimedReward,
    pub signature: String,
    pub block_time: Option<i64>,
}

impl SolanaClient {
    pub fn new(rpc_url: &str, program_id: Pubkey) -> Self {
        Self {
            client: RpcClient::new(rpc_url),
            program_id,
        }
    }

    pub fn get_claim_reward(
        &self,
        signature: &RpcConfirmedTransactionStatusWithSignature,
    ) -> Option<ClaimTx> {
        if signature.err.is_none() {
            match self.client.get_transaction(
                &Signature::from_str(&signature.signature).unwrap(),
                UiTransactionEncoding::Json,
            ) {
                Ok(tx) => {
                    if let Some(meta) = &tx.transaction.meta {
                        if let OptionSerializer::Some(logs) = &meta.log_messages {
                            let mut claim_reward = false;
                            for log in logs {
                                if log.starts_with("Program log:") && log.contains("ClaimReward") {
                                    claim_reward = true;
                                }
                                if log.starts_with("Program data:") {
                                    let program_data = &log[14..];
                                    if let Ok(decoded_bytes) =
                                        base64::engine::general_purpose::STANDARD
                                            .decode(program_data)
                                    {
                                        match ClaimedReward::deserialize(
                                            &mut decoded_bytes.as_ref(),
                                        ) {
                                            Ok(event) if claim_reward => {
                                                if &event.discriminator == b"claim" {
                                                    return Some(ClaimTx {
                                                        reward: event,
                                                        signature: signature.signature.clone(),
                                                        block_time: signature.block_time,
                                                    });
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => {}
            }
        }
        None
    }

    pub fn get_transactions(
        &self,
        latest_transaction_signature: Option<String>,
    ) -> Result<(Vec<ClaimTx>, Option<String>), anyhow::Error> {
        let latest_transaction_signature = latest_transaction_signature
            .as_ref()
            .and_then(|tx| Signature::from_str(tx).ok());
        let mut current_signature: Option<Signature> = None;
        let mut claim_txs = vec![];
        let mut new_latest_transaction_signature = None;
        loop {
            let sigs = self.client.get_signatures_for_address_with_config(
                &self.program_id,
                GetConfirmedSignaturesForAddress2Config {
                    before: current_signature,
                    until: latest_transaction_signature,
                    limit: Some(2),
                    ..Default::default()
                },
            );
            if let Ok(sigs) = sigs {
                if sigs.len() == 0 {
                    break;
                }
                if new_latest_transaction_signature.is_none() {
                    new_latest_transaction_signature = sigs.first().map(|tx| tx.signature.clone());
                }
                current_signature = sigs
                    .last()
                    .map(|tx| tx.signature.clone())
                    .as_ref()
                    .and_then(|tx| Signature::from_str(tx).ok());

                for sig in &sigs {
                    if let Some(claim_tx) = self.get_claim_reward(sig) {
                        claim_txs.push(claim_tx);
                    }
                }
            }
        }

        Ok((claim_txs, new_latest_transaction_signature))
    }
}

pub async fn run(service: Arc<AppService>, env: Env) -> Result<(), anyhow::Error> {
    let twitter_client = TwitterClient::new(
        env.twitter_bearer_token,
        env.twitter_access_token,
        env.twitter_access_token_secret,
        env.twitter_api_key,
        env.twitter_api_key_secret,
    );
    let mut media_id = None;

    let client = SolanaClient::new(&env.solana_rpc_url, snake_contract::ID);

    let latest_transaction_signature = service.util.get_latest_transaction_signature().await?;

    // Fetch new tweets with author information
    let (claim_txs, latest_transaction_signature) =
        client.get_transactions(latest_transaction_signature)?;

    for claim_tx in &claim_txs {
        if let Ok(Some(user)) = service
            .user
            .get_user_by_wallet_address(&claim_tx.reward.user.to_string())
            .await
        {
            let block_time = claim_tx
                .block_time
                .and_then(|block_time| DateTime::from_timestamp(block_time, 0))
                .unwrap_or_else(Utc::now);

            // Update latest_claim_timestamp on user table
            service
                .user
                .set_latest_claim_timestamp(&user.id, &block_time)
                .await
                .ok();

            // Update rewards table with blockchain data
            if let Ok(Some(reward)) = service.reward.get_available_reward(&user.id).await {
                service
                    .reward
                    .update_reward(
                        &reward.id,
                        &claim_tx.signature,
                        claim_tx.reward.reward_amount,
                        &claim_tx.reward.user.to_string(),
                        &block_time,
                        false,
                    )
                    .await
                    .ok();

                if media_id.is_none() {
                    media_id = match twitter_client.upload_media("./gifs/playsnake.gif").await {
                        Ok((media_id, _)) => Some(media_id),
                        Err(err) => {
                            println!("uploading media: {:?}", err);
                            continue;
                        }
                    };
                }

                match twitter_client
                    .reply_with_media("", &media_id, &reward.tweet_twitter_id)
                    .await
                {
                    Ok(_) => {}
                    Err(err) => {
                        println!("send dead snake gif error: {:?}", err);
                    }
                }
            }
        }
    }

    // Update latest_transaction_signature from response
    if let Some(latest_transaction_signature) = latest_transaction_signature {
        service
            .util
            .upsert_latest_transaction_signature(&latest_transaction_signature)
            .await?;
    }

    Ok(())
}
