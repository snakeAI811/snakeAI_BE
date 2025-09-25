use anchor_client::solana_sdk::system_program;
use anchor_lang::prelude::*;
use anyhow::Result;
use crate::AppState;

pub async fn initialize_reward_pool_backend(app_state: &AppState) -> Result<()> {
    let program = &app_state.program;

    let (reward_pool_pda, _bump) = Pubkey::find_program_address(&[b"reward_pool"], &program.id());

    program
        .request()
        .accounts(snake_contract::accounts::InitializeRewardPool {
            reward_pool: reward_pool_pda,
            admin: program.payer(), // backend wallet
            system_program: system_program::ID,
        })
        .args(snake_contract::instruction::InitializeRewardPool {})
        .send()?;

    println!("✅ Reward pool initialized at: {}", reward_pool_pda);
    Ok(())
}
