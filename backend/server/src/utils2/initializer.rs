use anchor_client::{
    solana_sdk::{
        pubkey::Pubkey,
    },
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::ID as TOKEN_PROGRAM_ID;
use anyhow::Result;
use std::str::FromStr;

use crate::state::AppState;
use snake_contract::instruction::InitializeRewardPool as InitializeRewardPoolIx;
use snake_contract::accounts::InitializeRewardPool;

pub async fn initialize_reward_pool_backend(state: &AppState) -> Result<()> {
    let program = &state.program;

    // Backend wallet (owner)
    let owner = program.payer();
    let owner_pubkey = owner;

    // Token mint address
    let mint = Pubkey::from_str(&state.env.token_mint)?;
    let owner_ata = get_associated_token_address(&owner_pubkey, &mint);

    // Derive reward pool PDA (must match seeds used in #[account])
    let (reward_pool_pda, _bump) =
        Pubkey::find_program_address(&[b"reward_pool"], &program.id());

    // Treasury ATA for reward_pool PDA
    let treasury = get_associated_token_address(&reward_pool_pda, &mint);

    // ✅ Log all inputs
    println!("--- 🧾 Initializing Reward Pool ---");
    println!("Owner Pubkey: {}", owner_pubkey);
    println!("Mint: {}", mint);
    println!("Owner ATA: {}", owner_ata);
    println!("Reward Pool PDA: {}", reward_pool_pda);
    println!("Treasury ATA: {}", treasury);
    println!("----------------------------------");

    // Run the blocking operation in a separate thread
    let program_clone = program.clone();
    tokio::task::spawn_blocking(move || {
        program_clone
            .request()
            .accounts(InitializeRewardPool {
                owner: owner_pubkey,
                owner_ata,
                reward_pool: reward_pool_pda,
                treasury,
                mint,
                associated_token_program: spl_associated_token_account::ID,
                token_program: TOKEN_PROGRAM_ID,
                system_program: anchor_client::solana_sdk::system_program::ID,
            })
            .args(InitializeRewardPoolIx { 
                args: snake_contract::instructions::initialize_reward_pool::InitializeRewardPoolParams { 
                    admin: owner_pubkey, 
                }, 
            }) 
            .send() 
    }).await??;

    println!("✅ Reward pool initialized");

    Ok(())
}

