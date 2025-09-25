// Explicitly import the global core to prevent shadowing
use ::core;
use anchor_lang::prelude::*;

pub mod constants;
mod errors;
mod events;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;
use state::UserRole;
use state::SwapType; // Use the state module's SwapType
use instructions::update_user_stats::UpdateUserStatsParams;


declare_id!("3sXaMR5bCoP5ePizVUCXcWykZL3PdckHMUKoG7gZyRY6");

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

    pub fn log_claim(ctx: Context<LogClaim>) -> Result<()> {
        instructions::log_claim(ctx)
    }

    pub fn batch_claim(ctx: Context<BatchClaim>) -> Result<()> {
        instructions::batch_claim(ctx)
    }

    pub fn select_role(ctx: Context<SelectRole>, role: UserRole) -> Result<()> {
        instructions::select_role(ctx, role)
    }

    pub fn claim_tokens_with_role(ctx: Context<ClaimTokensWithRole>, amount: u64, role: UserRole, tweet_id: String) -> Result<()> {
        instructions::claim_tokens_with_role(ctx, amount, role, tweet_id)
    }

    pub fn initialize_user_claim(ctx: Context<InitializeUserClaim>) -> Result<()> {
        instructions::initialize_user_claim(ctx)
    }

    // Patron-related functions
    pub fn apply_for_patron(ctx: Context<ApplyForPatron>, wallet_age_days: u32, community_score: u32) -> Result<()> {
        instructions::apply_for_patron(ctx, wallet_age_days, community_score)
    }

    pub fn approve_patron_application(ctx: Context<ApprovePatronApplication>, min_qualification_score: u32) -> Result<()> {
        instructions::approve_patron_application(ctx, min_qualification_score)
    }

    pub fn revoke_patron_status(ctx: Context<ApprovePatronApplication>) -> Result<()> {
        instructions::revoke_patron_status(ctx)
    }

    pub fn check_patron_eligibility(ctx: Context<ApplyForPatron>, min_score: u32) -> Result<bool> {
        instructions::check_patron_eligibility(ctx, min_score)
    }

    // Lock and staking functions
    pub fn lock_tokens(ctx: Context<LockTokens>, amount: u64, duration_months: u8) -> Result<()> {
        instructions::lock_tokens(ctx, amount, duration_months)
    }

    pub fn unlock_tokens(ctx: Context<UnlockTokens>) -> Result<()> {
        instructions::unlock_tokens(ctx)
    }

    pub fn claim_yield(ctx: Context<ClaimYield>) -> Result<()> {
        instructions::claim_yield(ctx)
    }

    // ========== STAKING HISTORY TRACKING FUNCTIONS ==========
    
    pub fn initialize_staking_history(ctx: Context<InitializeStakingHistory>) -> Result<()> {
        instructions::initialize_staking_history(ctx)
    }

    pub fn initialize_global_stats(ctx: Context<InitializeGlobalStats>) -> Result<()> {
        instructions::initialize_global_stats(ctx)
    }

    pub fn get_user_staking_summary(ctx: Context<GetStakingHistory>) -> Result<()> {
        instructions::get_user_staking_summary(ctx)
    }

    pub fn get_global_staking_stats(ctx: Context<GetGlobalStats>) -> Result<()> {
        instructions::get_global_staking_stats(ctx)
    }

    // ========== NEW PATRON FRAMEWORK FUNCTIONS ==========

    // OTC Swap functions using the new optimized module
    pub fn initiate_otc_swap(
        ctx: Context<InitiateOtcSwap>, 
        token_amount: u64, 
        sol_rate: u64,
        buyer_rebate: u64,
        swap_type: SwapType
    ) -> Result<()> {
        instructions::otc_swap::initiate_swap(ctx, token_amount, sol_rate, buyer_rebate, swap_type)
    }

    pub fn accept_otc_swap(
        ctx: Context<AcceptOtcSwap>,
        buyer_rebate: u64,
    ) -> Result<()> {
        instructions::otc_swap::accept_swap(ctx, buyer_rebate)
    }

    pub fn cancel_otc_swap(
        ctx: Context<CancelOtcSwap>,
    ) -> Result<()> {
        instructions::otc_swap::cancel_swap(ctx)
    }

    pub fn patron_exit(ctx: Context<PatronExit>, exit_amount: u64) -> Result<()> {
        instructions::patron_exit(ctx, exit_amount)
    }

    pub fn patron_otc_exit(
        ctx: Context<PatronOTCExit>,
        exit_amount: u64,
        sale_price: u64,
    ) -> Result<()> {
        instructions::patron_otc_exit(ctx, exit_amount, sale_price)
    }

    pub fn create_vesting_schedule(
        ctx: Context<CreateVesting>,
        vesting_amount: u64,
    ) -> Result<()> {
        instructions::vesting::create_vesting_schedule(ctx, vesting_amount)
    }

    pub fn claim_vested_tokens(ctx: Context<WithdrawVesting>) -> Result<()> {
        instructions::vesting::claim_vested_tokens(ctx)
    }

    pub fn allocate_dao_seat(
        ctx: Context<AllocateDAOSeat>,
        current_balance: u64,
    ) -> Result<()> {
        instructions::allocate_dao_seat(ctx, current_balance)
    }

    pub fn revoke_dao_seat(ctx: Context<RevokeDAOSeat>, reason: String) -> Result<()> {
        instructions::revoke_dao_seat(ctx, reason)
    }

    pub fn initialize_dao_registry(
        ctx: Context<InitializeDAORegistry>,
        max_seats: u32,
        min_dao_stake: u64,
        month6_timestamp: i64,
    ) -> Result<()> {
        instructions::initialize_dao_registry(ctx, max_seats, min_dao_stake, month6_timestamp)
    }
    
    // ========== OTC SWAP TRACKING & DEFLATIONARY MECHANICS ==========
    
    pub fn track_otc_swap(
        ctx: Context<TrackOtcSwap>,
        amount: u64,
        is_sale: bool,
    ) -> Result<()> {
        instructions::otc_swap::track_swap_operation(ctx, amount, is_sale)
    }

    pub fn apply_burn_penalty(
        ctx: Context<ApplyBurnPenalty>,
        burn_amount: u64,
    ) -> Result<()> {
        instructions::otc_swap::apply_burn_penalty(ctx, burn_amount)
    }

    pub fn revoke_dao_eligibility(
        ctx: Context<RevokeDAOEligibility>,
    ) -> Result<()> {
        instructions::otc_swap::revoke_dao_eligibility(ctx)
    }

    pub fn get_swap_tracker_stats(ctx: Context<GetSwapTrackerStats>) -> Result<(u64, u64, i64, bool, bool, bool)> {
        instructions::otc_swap::get_swap_tracker_stats(ctx)
    }

   
    /// Update user statistics for patron qualification
    pub fn update_user_stats(
        ctx: Context<UpdateUserStats>,
        params: UpdateUserStatsParams,
    ) -> Result<()> {
        instructions::update_user_stats(ctx, params)
    }

    // ========== TCE (Token Claim Event) ==========
    
    /// Start the Token Claim Event - allows users to claim accumulated rewards
    pub fn start_tce(ctx: Context<StartTce>) -> Result<()> {
        instructions::start_tce(ctx)
    }

    /// Update a user's accumulated rewards (Admin only)
    /// Used to sync off-chain rewards to on-chain during TCE
    pub fn update_accumulated_rewards(
        ctx: Context<UpdateAccumulatedRewards>,
        amount: u64,
    ) -> Result<()> {
        instructions::update_accumulated_rewards(ctx, amount)
    }
   
}