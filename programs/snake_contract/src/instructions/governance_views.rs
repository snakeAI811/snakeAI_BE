use anchor_lang::prelude::*;
use crate::state::{
    DaoRegistry, Proposal, Vote, ProposalStatus, ProposalType,
    UserClaim, UserRole, PatronStatus
};

// ========== MILESTONE 3: GOVERNANCE VIEW FUNCTIONS ==========

// ========== GET PROPOSAL INFO ==========

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct GetProposalInfo<'info> {
    #[account(
        seeds = [b"proposal", proposal_id.to_le_bytes().as_ref()],
        bump = proposal.bump,
    )]
    pub proposal: Account<'info, Proposal>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ProposalInfo {
    pub id: u64,
    pub proposer: Pubkey,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub status: ProposalStatus,
    pub created_at: i64,
    pub voting_ends_at: i64,
    pub executed_at: i64,
    pub votes_for: u32,
    pub votes_against: u32,
    pub total_votes: u32,
    pub quorum_reached: bool,
    pub target_account: Option<Pubkey>,
    pub amount: Option<u64>,
    pub new_value: Option<u64>,
}

pub fn get_proposal_info(ctx: Context<GetProposalInfo>) -> Result<ProposalInfo> {
    let proposal = &ctx.accounts.proposal;
    
    Ok(ProposalInfo {
        id: proposal.id,
        proposer: proposal.proposer,
        title: proposal.title.clone(),
        description: proposal.description.clone(),
        proposal_type: proposal.proposal_type.clone(),
        status: proposal.status.clone(),
        created_at: proposal.created_at,
        voting_ends_at: proposal.voting_ends_at,
        executed_at: proposal.executed_at,
        votes_for: proposal.votes_for,
        votes_against: proposal.votes_against,
        total_votes: proposal.total_votes,
        quorum_reached: proposal.quorum_reached,
        target_account: proposal.target_account,
        amount: proposal.amount,
        new_value: proposal.new_value,
    })
}

// ========== GET GOVERNANCE INFO ==========

#[derive(Accounts)]
pub struct GetGovernanceInfo<'info> {
    #[account(
        seeds = [b"dao_registry"],
        bump,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct GovernanceInfo {
    pub total_seats: u32,
    pub occupied_seats: u32,
    pub proposal_count: u64,
    pub voting_period_days: u8,
    pub quorum_threshold: u32,
    pub approval_threshold: u32,
    pub proposal_deposit: u64,
    pub governance_active: bool,
    pub min_stake_for_eligibility: u64,
}

pub fn get_governance_info(ctx: Context<GetGovernanceInfo>) -> Result<GovernanceInfo> {
    let dao = &ctx.accounts.dao_registry;
    
    Ok(GovernanceInfo {
        total_seats: dao.total_seats,
        occupied_seats: dao.occupied_seats,
        proposal_count: dao.proposal_count,
        voting_period_days: dao.voting_period_days,
        quorum_threshold: dao.quorum_threshold,
        approval_threshold: dao.approval_threshold,
        proposal_deposit: dao.proposal_deposit,
        governance_active: dao.governance_active,
        min_stake_for_eligibility: dao.min_stake_for_eligibility,
    })
}

// ========== GET USER VOTE ==========

#[derive(Accounts)]
#[instruction(proposal_id: u64, voter: Pubkey)]
pub struct GetUserVote<'info> {
    #[account(
        seeds = [b"vote", proposal_id.to_le_bytes().as_ref(), voter.as_ref()],
        bump = vote.bump,
    )]
    pub vote: Account<'info, Vote>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UserVoteInfo {
    pub proposal_id: u64,
    pub voter: Pubkey,
    pub vote_for: bool,
    pub voting_power: u32,
    pub voted_at: i64,
}

pub fn get_user_vote(ctx: Context<GetUserVote>) -> Result<UserVoteInfo> {
    let vote = &ctx.accounts.vote;
    
    Ok(UserVoteInfo {
        proposal_id: vote.proposal_id,
        voter: vote.voter,
        vote_for: vote.vote_for,
        voting_power: vote.voting_power,
        voted_at: vote.voted_at,
    })
}

// ========== CHECK VOTING ELIGIBILITY ==========

#[derive(Accounts)]
pub struct CheckVotingEligibility<'info> {
    /// CHECK: This is the user whose voting eligibility is being checked
    pub user: UncheckedAccount<'info>,
    
    #[account(
        seeds = [b"user_claim", user.key().as_ref()],
        bump,
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        seeds = [b"dao_registry"],
        bump,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct VotingEligibilityInfo {
    pub is_eligible: bool,
    pub has_dao_seat: bool,
    pub voting_power: u32,
    pub reason: String,
}

pub fn check_voting_eligibility(ctx: Context<CheckVotingEligibility>) -> Result<VotingEligibilityInfo> {
    let user_claim = &ctx.accounts.user_claim;
    let dao_registry = &ctx.accounts.dao_registry;
    
    let is_eligible = user_claim.dao_seat_holder && dao_registry.governance_active;
    let voting_power = if is_eligible {
        1 + (user_claim.locked_amount / 1000) as u32 // Base vote + token weight
    } else {
        0
    };
    
    let reason = if !dao_registry.governance_active {
        "Governance is not active".to_string()
    } else if !user_claim.dao_seat_holder {
        "User does not hold a DAO seat".to_string()
    } else {
        "Eligible to vote".to_string()
    };
    
    Ok(VotingEligibilityInfo {
        is_eligible,
        has_dao_seat: user_claim.dao_seat_holder,
        voting_power,
        reason,
    })
}

// ========== GET ACTIVE PROPOSALS ==========

#[derive(Accounts)]
pub struct GetActiveProposals<'info> {
    #[account(
        seeds = [b"dao_registry"],
        bump,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ActiveProposalsInfo {
    pub total_proposals: u64,
    pub governance_active: bool,
    pub current_timestamp: i64,
}

pub fn get_active_proposals(ctx: Context<GetActiveProposals>) -> Result<ActiveProposalsInfo> {
    let dao = &ctx.accounts.dao_registry;
    let current_time = Clock::get()?.unix_timestamp;
    
    Ok(ActiveProposalsInfo {
        total_proposals: dao.proposal_count,
        governance_active: dao.governance_active,
        current_timestamp: current_time,
    })
}

// ========== GET PROPOSAL VOTING STATS ==========

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct GetProposalVotingStats<'info> {
    #[account(
        seeds = [b"proposal", proposal_id.to_le_bytes().as_ref()],
        bump = proposal.bump,
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        seeds = [b"dao_registry"],
        bump,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ProposalVotingStats {
    pub proposal_id: u64,
    pub votes_for: u32,
    pub votes_against: u32,
    pub total_votes: u32,
    pub quorum_required: u32,
    pub quorum_reached: bool,
    pub approval_percentage: u32,
    pub approval_threshold: u32,
    pub voting_ends_at: i64,
    pub time_remaining: i64,
    pub can_be_finalized: bool,
}

pub fn get_proposal_voting_stats(ctx: Context<GetProposalVotingStats>) -> Result<ProposalVotingStats> {
    let proposal = &ctx.accounts.proposal;
    let dao = &ctx.accounts.dao_registry;
    let current_time = Clock::get()?.unix_timestamp;
    
    let quorum_required = dao.calculate_quorum_required();
    let approval_percentage = if proposal.total_votes > 0 {
        (proposal.votes_for * 100) / proposal.total_votes
    } else {
        0
    };
    
    let time_remaining = if current_time < proposal.voting_ends_at {
        proposal.voting_ends_at - current_time
    } else {
        0
    };
    
    let can_be_finalized = proposal.status == ProposalStatus::Active && 
                          current_time > proposal.voting_ends_at;
    
    Ok(ProposalVotingStats {
        proposal_id: proposal.id,
        votes_for: proposal.votes_for,
        votes_against: proposal.votes_against,
        total_votes: proposal.total_votes,
        quorum_required,
        quorum_reached: proposal.quorum_reached,
        approval_percentage,
        approval_threshold: dao.approval_threshold,
        voting_ends_at: proposal.voting_ends_at,
        time_remaining,
        can_be_finalized,
    })
}