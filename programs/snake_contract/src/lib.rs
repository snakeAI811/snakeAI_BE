use anchor_lang::prelude::*;

mod constants;
mod errors;
mod events;
mod instructions;
mod state;

use instructions::*;
use instructions::{SelectRole, ClaimTokensWithRole, InitiateOtcSwap, AcceptOtcSwap, SellbackToProject, InitializeUserClaim};
use state::UserRole;

declare_id!("Aw4zQtbMuxCChXS923HeyhAMPakC8KMQa6tQmMS4pPYM");

#[program]
pub mod snake_contract {
    use super::*;

    pub fn initialize_reward_pool(
        ctx: Context<InitializeRewardPool>,
        args: InitializeRewardPoolParams,
    ) -> Result<()> {
        instructions::initialize_reward_pool(ctx, args)
    }

    pub fn update_reward_pool(
        ctx: Context<UpdateRewardPool>,
        args: UpdateRewardPoolParams,
    ) -> Result<()> {
        instructions::update_reward_pool(ctx, args)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        instructions::claim_reward(ctx)
    }

    pub fn select_role(ctx: Context<SelectRole>, role: UserRole) -> Result<()> {
        instructions::select_role(ctx, role)
    }

    pub fn claim_tokens_with_role(ctx: Context<ClaimTokensWithRole>, amount: u64, role: UserRole) -> Result<()> {
        instructions::claim_tokens_with_role(ctx, amount, role)
    }

    pub fn initiate_otc_swap(ctx: Context<InitiateOtcSwap>, amount: u64, rate: u64, patron_rebate: u64) -> Result<()> {
        instructions::initiate_otc_swap(ctx, amount, rate, patron_rebate)
    }

    pub fn accept_otc_swap(ctx: Context<AcceptOtcSwap>) -> Result<()> {
        instructions::accept_otc_swap(ctx)
    }

    pub fn cancel_otc_swap(ctx: Context<CancelOtcSwap>) -> Result<()> {
        instructions::cancel_otc_swap(ctx)
    }

    pub fn sellback_to_project(ctx: Context<SellbackToProject>, amount: u64) -> Result<()> {
        instructions::sellback_to_project(ctx, amount)
    }

    pub fn initialize_user_claim(ctx: Context<InitializeUserClaim>) -> Result<()> {
        instructions::initialize_user_claim(ctx)
    }
}
