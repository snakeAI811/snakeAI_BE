use anchor_lang::prelude::*;

mod constants;
mod errors;
mod events;
mod instructions;
mod state;

use instructions::*;

declare_id!("Aw4zQtbMuxCChXS923HeyhAMPakC8KMQa6tQmMS4pPYM");

#[program]
pub mod snake_contract {
    use super::*;

    pub fn initialize_reward_pool(ctx: Context<InitializeRewardPool>) -> Result<()> {
        instructions::initialize_reward_pool(ctx)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        instructions::claim_reward(ctx)
    }
}
