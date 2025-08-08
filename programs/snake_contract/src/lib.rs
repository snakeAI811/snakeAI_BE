use anchor_lang::prelude::*;

pub mod constants;
mod errors;
mod events;
pub mod instructions;
pub mod state;

use instructions::*;
use state::UserRole;
use instructions::otc_swap_enhanced::SwapType;
use instructions::otc_trading;
use instructions::update_user_stats::UpdateUserStatsParams;

declare_id!("A75BhFuMiJoe3bwNXAJbsPykKSQfvVhC5oYH9k9X6RQy");

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

    pub fn claim_tokens_with_role(ctx: Context<ClaimTokensWithRole>, amount: u64, role: UserRole, tweet_id: String) -> Result<()> {
        instructions::claim_tokens_with_role(ctx, amount, role, tweet_id)
    }

    pub fn initialize_user_claim(ctx: Context<InitializeUserClaim>) -> Result<()> {
        instructions::initialize_user_claim(ctx)
    }

    pub fn accept_otc_swap(ctx: Context<AcceptOtcSwap>) -> Result<()> {
        instructions::accept_otc_swap(ctx)
    }

    pub fn cancel_otc_swap(ctx: Context<CancelOtcSwap>) -> Result<()> {
        instructions::cancel_otc_swap(ctx)
    }


    // pub fn sellback_to_project(ctx: Context<SellbackToProject>, amount: u64) -> Result<()> {
    //     instructions::sellback_to_project(ctx, amount)
    // }

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

    // ========== NEW PATRON FRAMEWORK FUNCTIONS ==========

    pub fn create_otc_order(
        ctx: Context<CreateOTCOrder>,
        order_id: u64,
        amount: u64,
        price: u64,
        buyer_restrictions: otc_trading::BuyerRestrictions,
    ) -> Result<()> {
        instructions::create_otc_order(ctx, order_id, amount, price, buyer_restrictions)
    }

    pub fn execute_otc_order(ctx: Context<ExecuteOTCOrder>) -> Result<()> {
        instructions::execute_otc_order(ctx)
    }

    pub fn cancel_otc_order(ctx: Context<CancelOTCOrder>) -> Result<()> {
        instructions::cancel_otc_order(ctx)
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
    
    pub fn initiate_otc_swap_enhanced(
        ctx: Context<InitiateOtcSwapEnhanced>, 
        token_amount: u64, 
        sol_rate: u64,
        buyer_rebate: u64,
        swap_type: SwapType
    ) -> Result<()> {
        instructions::initiate_otc_swap_enhanced(ctx, token_amount, sol_rate, buyer_rebate, swap_type)
    }

    pub fn accept_otc_swap_patron_to_patron(ctx: Context<AcceptOtcSwapPatronToPatron>) -> Result<()> {
        instructions::accept_otc_swap_patron_to_patron(ctx)
    }

    pub fn accept_treasury_buyback(ctx: Context<AcceptTreasuryBuyback>) -> Result<()> {
        instructions::accept_treasury_buyback(ctx)
    }

    // ========== STUB OTC SWAP/BURN LOGIC ==========
    
    pub fn simulate_otc_swap(
        ctx: Context<SimulateOtcSwap>,
        amount: u64,
        is_sale: bool,
    ) -> Result<()> {
        instructions::simulate_otc_swap(ctx, amount, is_sale)
    }

    pub fn simulate_burn_on_exit(
        ctx: Context<SimulateBurnOnExit>,
        exit_amount: u64,
    ) -> Result<()> {
        instructions::simulate_burn_on_exit(ctx, exit_amount)
    }

    pub fn real_burn_on_exit(
        ctx: Context<RealBurnOnExit>,
        exit_amount: u64,
    ) -> Result<()> {
        instructions::real_burn_on_exit(ctx, exit_amount)
    }

    pub fn get_swap_stats(ctx: Context<GetSwapStats>) -> Result<(u64, u64, i64, bool, bool, bool)> {
        instructions::get_swap_stats(ctx)
    }

    pub fn mock_ui_event(
        ctx: Context<MockUIEvent>,
        event_type: String,
        amount: u64,
    ) -> Result<()> {
        instructions::mock_ui_event(ctx, event_type, amount)
    }

   
    /// Update user statistics for patron qualification
    pub fn update_user_stats(
        ctx: Context<UpdateUserStats>,
        params: UpdateUserStatsParams,
    ) -> Result<()> {
        instructions::update_user_stats(ctx, params)
    }
   
}

