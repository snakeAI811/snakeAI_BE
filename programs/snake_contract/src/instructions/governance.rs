use anchor_lang::prelude::*;
use crate::state::{
    DaoRegistry, UserClaim, UserRole, PatronStatus, 
    Proposal, Vote, ProposalType, ProposalStatus
};
use crate::errors::SnakeError;
use crate::events::{
    ProposalCreated, VoteCast, ProposalFinalized, ProposalExecuted, ProposalCancelled
};

// ========== MILESTONE 3: GOVERNANCE SYSTEM ==========

// ========== CREATE PROPOSAL ==========

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,
    
    #[account(
        constraint = proposer_claim.initialized @ SnakeError::Unauthorized,
        constraint = proposer_claim.dao_seat_holder @ SnakeError::NotDAOSeatHolder,
        seeds = [b"user_claim", proposer.key().as_ref()],
        bump,
    )]
    pub proposer_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [b"dao_registry"],
        bump,
        constraint = dao_registry.is_governance_active() @ SnakeError::GovernanceNotActive,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
    
    #[account(
        init,
        payer = proposer,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [b"proposal", dao_registry.proposal_count.checked_add(1).unwrap().to_le_bytes().as_ref()],
        bump,
    )]
    pub proposal: Account<'info, Proposal>,
    
    pub system_program: Program<'info, System>,
}

pub fn create_proposal(
    ctx: Context<CreateProposal>,
    title: String,
    description: String,
    proposal_type: ProposalType,
    target_account: Option<Pubkey>,
    amount: Option<u64>,
    new_value: Option<u64>,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    // Validate proposal parameters
    require!(title.len() <= 100, SnakeError::InvalidProposalTitle);
    require!(description.len() <= 500, SnakeError::InvalidProposalDescription);
    
    // Check if proposer has sufficient SOL for deposit
    require!(
        ctx.accounts.proposer.to_account_info().lamports() >= ctx.accounts.dao_registry.proposal_deposit,
        SnakeError::InsufficientFunds
    );
    
    let dao_registry = &mut ctx.accounts.dao_registry;
    let proposal_id = dao_registry.next_proposal_id();
    
    let proposal = &mut ctx.accounts.proposal;
    proposal.init(
        proposal_id,
        ctx.accounts.proposer.key(),
        title.clone(),
        description.clone(),
        proposal_type.clone(),
        current_time,
        dao_registry.voting_period_days,
        ctx.bumps.proposal,
    );
    
    // Set proposal-specific data
    proposal.target_account = target_account;
    proposal.amount = amount;
    proposal.new_value = new_value;
    
    // Collect proposal deposit
    **ctx.accounts.proposer.to_account_info().try_borrow_mut_lamports()? -= dao_registry.proposal_deposit;
    
    emit!(ProposalCreated {
        proposal_id,
        proposer: ctx.accounts.proposer.key(),
        title,
        description,
        proposal_type,
        voting_ends_at: proposal.voting_ends_at,
    });
    
    Ok(())
}

// ========== CAST VOTE ==========

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    
    #[account(
        constraint = voter_claim.initialized @ SnakeError::Unauthorized,
        constraint = voter_claim.dao_seat_holder @ SnakeError::NotDAOSeatHolder,
        seeds = [b"user_claim", voter.key().as_ref()],
        bump,
    )]
    pub voter_claim: Account<'info, UserClaim>,
    
    #[account(
        seeds = [b"dao_registry"],
        bump,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
    
    #[account(
        mut,
        seeds = [b"proposal", proposal_id.to_le_bytes().as_ref()],
        bump = proposal.bump,
        constraint = proposal.id == proposal_id @ SnakeError::InvalidProposal,
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        init,
        payer = voter,
        space = 8 + Vote::INIT_SPACE,
        seeds = [b"vote", proposal_id.to_le_bytes().as_ref(), voter.key().as_ref()],
        bump,
    )]
    pub vote: Account<'info, Vote>,
    
    pub system_program: Program<'info, System>,
}

pub fn cast_vote(
    ctx: Context<CastVote>,
    proposal_id: u64,
    vote_for: bool,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if voting is still active
    require!(
        ctx.accounts.proposal.is_voting_active(current_time),
        SnakeError::VotingPeriodEnded
    );
    
    // Calculate voting power (seat-based + token-weighted)
    let base_voting_power = 1; // 1 vote per seat
    let token_weight = ctx.accounts.voter_claim.locked_amount / 1000; // 1 additional vote per 1000 tokens
    let total_voting_power = base_voting_power + token_weight as u32;
    
    // Record the vote
    let vote = &mut ctx.accounts.vote;
    vote.init(
        proposal_id,
        ctx.accounts.voter.key(),
        vote_for,
        total_voting_power,
        current_time,
        ctx.bumps.vote,
    );
    
    // Update proposal vote counts
    let proposal = &mut ctx.accounts.proposal;
    proposal.add_vote(vote_for, total_voting_power);
    
    emit!(VoteCast {
        proposal_id,
        voter: ctx.accounts.voter.key(),
        vote_for,
        voting_power: total_voting_power,
    });
    
    Ok(())
}

// ========== FINALIZE PROPOSAL ==========

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct FinalizeProposal<'info> {
    #[account(mut)]
    pub finalizer: Signer<'info>,
    
    #[account(
        seeds = [b"dao_registry"],
        bump,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
    
    #[account(
        mut,
        seeds = [b"proposal", proposal_id.to_le_bytes().as_ref()],
        bump = proposal.bump,
        constraint = proposal.id == proposal_id @ SnakeError::InvalidProposal,
        constraint = proposal.status == ProposalStatus::Active @ SnakeError::ProposalNotActive,
    )]
    pub proposal: Account<'info, Proposal>,
}

pub fn finalize_proposal(
    ctx: Context<FinalizeProposal>,
    proposal_id: u64,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if voting period has ended
    require!(
        ctx.accounts.proposal.is_voting_ended(current_time),
        SnakeError::VotingPeriodNotEnded
    );
    
    // Finalize the proposal based on votes
    let proposal = &mut ctx.accounts.proposal;
    proposal.finalize_voting(&ctx.accounts.dao_registry);
    
    emit!(ProposalFinalized {
        proposal_id,
        status: proposal.status.clone(),
        votes_for: proposal.votes_for,
        votes_against: proposal.votes_against,
        quorum_reached: proposal.quorum_reached,
    });
    
    Ok(())
}

// ========== EXECUTE PROPOSAL ==========

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct ExecuteProposal<'info> {
    #[account(mut)]
    pub executor: Signer<'info>,
    
    #[account(
        constraint = executor_claim.initialized @ SnakeError::Unauthorized,
        constraint = executor_claim.dao_seat_holder @ SnakeError::NotDAOSeatHolder,
        seeds = [b"user_claim", executor.key().as_ref()],
        bump,
    )]
    pub executor_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [b"dao_registry"],
        bump,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
    
    #[account(
        mut,
        seeds = [b"proposal", proposal_id.to_le_bytes().as_ref()],
        bump = proposal.bump,
        constraint = proposal.id == proposal_id @ SnakeError::InvalidProposal,
        constraint = proposal.status == ProposalStatus::Passed @ SnakeError::ProposalNotPassed,
    )]
    pub proposal: Account<'info, Proposal>,
    
    /// CHECK: Target account for proposal execution (validated by proposal type)
    #[account(mut)]
    pub target_account: Option<UncheckedAccount<'info>>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn execute_proposal(
    ctx: Context<ExecuteProposal>,
    proposal_id: u64,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let proposal = &mut ctx.accounts.proposal;
    
    // Execute based on proposal type
    match proposal.proposal_type {
        ProposalType::TextProposal => {
            // Text proposals don't require execution, just mark as executed
        },
        ProposalType::ParameterChange => {
            // Update DAO parameters
            if let (Some(new_value), Some(_target)) = (proposal.new_value, proposal.target_account) {
                // This would update specific parameters based on the target
                // For now, we'll update governance parameters as an example
                ctx.accounts.dao_registry.update_governance_params(
                    None, // voting_period_days
                    Some(new_value as u32), // quorum_threshold
                    None, // approval_threshold
                    None, // proposal_deposit
                );
            }
        },
        ProposalType::TreasurySpend => {
            // Transfer SOL from treasury
            if let (Some(amount), Some(target)) = (proposal.amount, proposal.target_account) {
                require!(
                    ctx.accounts.treasury.to_account_info().lamports() >= amount,
                    SnakeError::InsufficientFunds
                );
                
                **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? -= amount;
                
                if let Some(target_account) = &ctx.accounts.target_account {
                    **target_account.to_account_info().try_borrow_mut_lamports()? += amount;
                }
            }
        },
        ProposalType::SeatManagement => {
            // Add or remove DAO seats
            if let Some(new_total) = proposal.new_value {
                ctx.accounts.dao_registry.total_seats = new_total as u32;
            }
        },
        ProposalType::EmergencyAction => {
            // Emergency actions would be handled case by case
            // This could include pausing contracts, emergency withdrawals, etc.
        }
    }
    
    // Mark proposal as executed
    proposal.execute(current_time);
    
    emit!(ProposalExecuted {
        proposal_id,
        executor: ctx.accounts.executor.key(),
        executed_at: current_time,
    });
    
    Ok(())
}

// ========== CANCEL PROPOSAL ==========

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct CancelProposal<'info> {
    #[account(mut)]
    pub canceller: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"proposal", proposal_id.to_le_bytes().as_ref()],
        bump = proposal.bump,
        constraint = proposal.id == proposal_id @ SnakeError::InvalidProposal,
        constraint = proposal.proposer == canceller.key() @ SnakeError::Unauthorized,
        constraint = proposal.status == ProposalStatus::Active @ SnakeError::ProposalNotActive,
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        seeds = [b"dao_registry"],
        bump,
    )]
    pub dao_registry: Account<'info, DaoRegistry>,
}

pub fn cancel_proposal(
    ctx: Context<CancelProposal>,
    proposal_id: u64,
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    // Only allow cancellation if voting hasn't started or very few votes
    require!(
        ctx.accounts.proposal.total_votes <= 1,
        SnakeError::CannotCancelProposal
    );
    
    // Cancel the proposal
    let proposal = &mut ctx.accounts.proposal;
    proposal.cancel();
    
    // Refund proposal deposit
    **ctx.accounts.canceller.to_account_info().try_borrow_mut_lamports()? += ctx.accounts.dao_registry.proposal_deposit;
    
    emit!(ProposalCancelled {
        proposal_id,
        canceller: ctx.accounts.canceller.key(),
        cancelled_at: current_time,
    });
    
    Ok(())
}