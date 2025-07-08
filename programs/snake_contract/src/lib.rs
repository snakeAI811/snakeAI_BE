use anchor_lang::prelude::*;

mod constants;
mod errors;
mod events;
mod instructions;
pub mod state;

use instructions::*;
use state::{UserRole, VestingRoleType};
use instructions::otc_swap_enhanced::SwapType;

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

    pub fn initialize_user_claim(ctx: Context<InitializeUserClaim>) -> Result<()> {
        instructions::initialize_user_claim(ctx)
    }

    pub fn initiate_otc_swap(ctx: Context<InitiateOtcSwap>, amount: u64, rate: u64, buyer_rebate: u64, buyer_role_required: UserRole) -> Result<()> {
        instructions::initiate_otc_swap(ctx, amount, rate, buyer_rebate, buyer_role_required)
    }

    pub fn accept_otc_swap(ctx: Context<AcceptOtcSwap>) -> Result<()> {
        instructions::accept_otc_swap(ctx)
    }

    pub fn cancel_otc_swap(ctx: Context<CancelOtcSwap>) -> Result<()> {
        instructions::cancel_otc_swap(ctx)
    }

    // ========== MILESTONE 1: BASIC OTC SWAPS ==========
    
    // /// Initiate basic OTC swap (Normal users → Patrons)
    // pub fn initiate_basic_otc_swap(
    //     ctx: Context<InitiateBasicOtcSwap>,
    //     token_amount: u64,
    //     sol_rate: u64,
    //     buyer_rebate: u64,
    //     buyer_role_required: UserRole,
    // ) -> Result<()> {
    //     instructions::initiate_basic_otc_swap(ctx, token_amount, sol_rate, buyer_rebate, buyer_role_required)
    // }

    // /// Accept basic OTC swap (Patrons only)
    // pub fn accept_basic_otc_swap(ctx: Context<AcceptBasicOtcSwap>) -> Result<()> {
    //     instructions::accept_basic_otc_swap(ctx)
    // }

    // /// Cancel basic OTC swap (Seller only)
    // pub fn cancel_basic_otc_swap(ctx: Context<CancelBasicOtcSwap>) -> Result<()> {
    //     instructions::cancel_basic_otc_swap(ctx)
    // }

    // ========== MILESTONE 2: ENHANCED OTC SWAPS ==========
    
    // /// Initiate enhanced OTC swap (supports all swap types)
    // pub fn initiate_otc_swap_enhanced(
    //     ctx: Context<InitiateOtcSwapEnhanced>, 
    //     token_amount: u64, 
    //     sol_rate: u64,
    //     buyer_rebate: u64,
    //     swap_type: SwapType
    // ) -> Result<()> {
    //     instructions::initiate_otc_swap_enhanced(ctx, token_amount, sol_rate, buyer_rebate, swap_type)
    // }

    // /// Accept Patron-to-Patron OTC swap (with 20% burn penalty)
    // pub fn accept_otc_swap_patron_to_patron(ctx: Context<AcceptOtcSwapPatronToPatron>) -> Result<()> {
    //     instructions::accept_otc_swap_patron_to_patron(ctx)
    // }

    // /// Treasury buyback (admin function)
    // pub fn accept_treasury_buyback(ctx: Context<AcceptTreasuryBuyback>) -> Result<()> {
    //     instructions::accept_treasury_buyback(ctx)
    // }

    // /// Fallback to treasury when swap expires
    // pub fn fallback_to_treasury(ctx: Context<FallbackToTreasury>) -> Result<()> {
    //     instructions::fallback_to_treasury(ctx)
    // }

    pub fn sellback_to_project(ctx: Context<SellbackToProject>, amount: u64) -> Result<()> {
        instructions::sellback_to_project(ctx, amount)
    }

    // pub fn initialize_user_claim(ctx: Context<InitializeUserClaim>) -> Result<()> {
    //     instructions::initialize_user_claim(ctx)
    // }

    // Patron-related functions
    // pub fn apply_as_patron(ctx: Context<ApplyAsPatron>) -> Result<()> {
    //     instructions::apply_as_patron(ctx)
    // }

    // pub fn confirm_patron(ctx: Context<ConfirmPatron>) -> Result<()> {
    //     instructions::confirm_patron(ctx)
    // }

    // pub fn exit_as_patron(ctx: Context<ExitAsPatron>, amount: u64) -> Result<()> {
    //     instructions::exit_as_patron(ctx, amount)
    // }

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

    // ========== VESTING PROGRAM ==========
    
    pub fn create_vesting(
        ctx: Context<CreateVesting>,
        amount: u64,
        role_type: VestingRoleType,
    ) -> Result<()> {
        instructions::create_vesting(ctx, amount, role_type)
    }

    pub fn withdraw_vesting(ctx: Context<WithdrawVesting>) -> Result<()> {
        instructions::withdraw_vesting(ctx)
    }

    pub fn admin_force_exit(ctx: Context<AdminForceExit>) -> Result<()> {
        instructions::admin_force_exit(ctx)
    }

    // ========== ENHANCED OTC SWAP PROGRAM ==========
    
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

    // DAO functions
    // pub fn initialize_dao_registry(ctx: Context<InitializeDAORegistry>, total_seats: Option<u32>, min_stake: Option<u64>) -> Result<()> {
    //     instructions::initialize_dao_registry(ctx, total_seats, min_stake)
    // }

    // pub fn acquire_dao_seat(ctx: Context<AcquireDAOSeat>) -> Result<()> {
    //     instructions::acquire_dao_seat(ctx)
    // }

    // pub fn transfer_dao_seat(ctx: Context<TransferDAOSeat>) -> Result<()> {
    //     instructions::transfer_dao_seat(ctx)
    // }

    // pub fn return_dao_seat(ctx: Context<ReturnDAOSeat>) -> Result<()> {
    //     instructions::return_dao_seat(ctx)
    // }

    // // View functions
    // pub fn check_dao_eligibility(ctx: Context<CheckDAOEligibility>) -> Result<bool> {
    //     instructions::check_dao_eligibility_view(ctx)
    // }

    // pub fn is_patron(ctx: Context<IsPatron>) -> Result<bool> {
    //     instructions::is_patron(ctx)
    // }

    // pub fn get_dao_info(ctx: Context<GetDAOMembers>) -> Result<(u32, u32, u64)> {
    //     instructions::get_dao_info(ctx)
    // }

    // pub fn get_user_info(ctx: Context<GetUserInfo>) -> Result<UserInfo> {
    //     instructions::get_user_info(ctx)
    // }

    // ========== MILESTONE 3: GOVERNANCE SYSTEM ==========
    
    // /// Create a new governance proposal
    // pub fn create_proposal(
    //     ctx: Context<CreateProposal>,
    //     title: String,
    //     description: String,
    //     proposal_type: ProposalType,
    //     target_account: Option<Pubkey>,
    //     amount: Option<u64>,
    //     new_value: Option<u64>,
    // ) -> Result<()> {
    //     instructions::create_proposal(ctx, title, description, proposal_type, target_account, amount, new_value)
    // }

    // /// Cast a vote on a proposal
    // pub fn cast_vote(
    //     ctx: Context<CastVote>,
    //     proposal_id: u64,
    //     vote_for: bool,
    // ) -> Result<()> {
    //     instructions::cast_vote(ctx, proposal_id, vote_for)
    // }

    // /// Finalize a proposal after voting period ends
    // pub fn finalize_proposal(
    //     ctx: Context<FinalizeProposal>,
    //     proposal_id: u64,
    // ) -> Result<()> {
    //     instructions::finalize_proposal(ctx, proposal_id)
    // }

    // /// Execute a passed proposal
    // pub fn execute_proposal(
    //     ctx: Context<ExecuteProposal>,
    //     proposal_id: u64,
    // ) -> Result<()> {
    //     instructions::execute_proposal(ctx, proposal_id)
    // }

    // /// Cancel a proposal (proposer only)
    // pub fn cancel_proposal(
    //     ctx: Context<CancelProposal>,
    //     proposal_id: u64,
    // ) -> Result<()> {
    //     instructions::cancel_proposal(ctx, proposal_id)
    // }

    // ========== GOVERNANCE VIEW FUNCTIONS ==========
    
    // /// Get detailed proposal information
    // pub fn get_proposal_info(ctx: Context<GetProposalInfo>) -> Result<ProposalInfo> {
    //     instructions::get_proposal_info(ctx)
    // }

    // /// Get governance system information
    // pub fn get_governance_info(ctx: Context<GetGovernanceInfo>) -> Result<GovernanceInfo> {
    //     instructions::get_governance_info(ctx)
    // }

    // /// Get user's vote on a specific proposal
    // pub fn get_user_vote(ctx: Context<GetUserVote>) -> Result<UserVoteInfo> {
    //     instructions::get_user_vote(ctx)
    // }

    // /// Check if user is eligible to vote
    // pub fn check_voting_eligibility(ctx: Context<CheckVotingEligibility>) -> Result<VotingEligibilityInfo> {
    //     instructions::check_voting_eligibility(ctx)
    // }

    // /// Get active proposals count and governance status
    // pub fn get_active_proposals(ctx: Context<GetActiveProposals>) -> Result<ActiveProposalsInfo> {
    //     instructions::get_active_proposals(ctx)
    // }

    // /// Get detailed voting statistics for a proposal
    // pub fn get_proposal_voting_stats(ctx: Context<GetProposalVotingStats>) -> Result<ProposalVotingStats> {
    //     instructions::get_proposal_voting_stats(ctx)
    // }

    // ========== USER STATS & PATRON QUALIFICATION ==========
    
    // /// Update user statistics for patron qualification
    // pub fn update_user_stats(
    //     ctx: Context<UpdateUserStats>,
    //     params: UpdateUserStatsParams,
    // ) -> Result<()> {
    //     instructions::update_user_stats(ctx, params)
    // }
    
    // /// Batch update multiple users (placeholder - use multiple single calls for now)
    // pub fn batch_update_user_stats(
    //     ctx: Context<BatchUpdateUserStats>,
    // ) -> Result<()> {
    //     instructions::batch_update_user_stats(ctx, vec![])
    // }

    // ========== DASHBOARD & SOCIAL LAYER ==========
    
    // /// Get user's public profile for social layer
    // pub fn get_user_profile(ctx: Context<GetUserProfile>) -> Result<UserPublicProfile> {
    //     instructions::get_user_profile(ctx)
    // }
    
    // /// Get patron-specific information
    // pub fn get_patron_info(ctx: Context<GetUserProfile>) -> Result<Option<PatronInfo>> {
    //     instructions::get_patron_info(ctx)
    // }
    
    // /// Get system statistics for dashboard
    // pub fn get_system_stats(ctx: Context<GetSystemStats>) -> Result<SystemStats> {
    //     instructions::get_system_stats(ctx)
    // }
    
    // /// Check if user is a patron (public view)
    // pub fn is_user_patron(ctx: Context<GetUserProfile>) -> Result<bool> {
    //     instructions::is_user_patron(ctx)
    // }
    
    // /// Get user's current qualification score
    // pub fn get_qualification_score(ctx: Context<GetUserProfile>) -> Result<u32> {
    //     instructions::get_qualification_score(ctx)
    // }
    
    // /// Check Month 6 DAO eligibility status
    // pub fn check_dao_eligibility_status(ctx: Context<GetUserProfile>) -> Result<bool> {
    //     instructions::check_dao_eligibility_status(ctx)
    // }
}

