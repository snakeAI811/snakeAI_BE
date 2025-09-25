use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
    Cancelled,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum ProposalType {
    TextProposal,           // General governance proposal
    ParameterChange,        // Change contract parameters
    TreasurySpend,         // Spend from treasury
    SeatManagement,        // Add/remove DAO seats
    EmergencyAction,       // Emergency governance action
}

#[account]
#[derive(Default, InitSpace)]
pub struct DaoRegistry {
    pub initialized: bool,
    pub total_seats: u32,
    pub occupied_seats: u32,
    pub min_stake_for_eligibility: u64,
    pub lock_duration_requirement_months: u8, // 6 months
    
    // ========== MILESTONE 3: GOVERNANCE FEATURES ==========
    pub proposal_count: u64,
    pub voting_period_days: u8,        // Default 7 days
    pub quorum_threshold: u32,         // Minimum seats required to vote (percentage)
    pub approval_threshold: u32,       // Percentage of votes needed to pass (e.g., 60%)
    pub proposal_deposit: u64,         // SOL required to create proposal
    pub governance_active: bool,       // Enable/disable governance
}

#[account]
#[derive(InitSpace)]
pub struct Proposal {
    pub id: u64,
    pub proposer: Pubkey,
    #[max_len(100)]
    pub title: String,                 // Max 100 chars
    #[max_len(500)]
    pub description: String,           // Max 500 chars
    pub proposal_type: ProposalType,
    pub status: ProposalStatus,
    pub created_at: i64,
    pub voting_ends_at: i64,
    pub executed_at: i64,
    
    // Voting results
    pub votes_for: u32,
    pub votes_against: u32,
    pub total_votes: u32,
    pub quorum_reached: bool,
    
    // Proposal-specific data
    pub target_account: Option<Pubkey>, // For parameter changes
    pub amount: Option<u64>,            // For treasury spends
    pub new_value: Option<u64>,         // For parameter changes
    #[max_len(1000)]
    pub execution_data: Vec<u8>,        // Custom execution data
    
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Vote {
    pub proposal_id: u64,
    pub voter: Pubkey,
    pub vote_for: bool,               // true = for, false = against
    pub voting_power: u32,            // Based on seat ownership and token weight
    pub voted_at: i64,
    pub bump: u8,
}

impl DaoRegistry {
    pub fn init(&mut self, total_seats: u32, min_stake: u64) {
        self.initialized = true;
        self.total_seats = total_seats;
        self.occupied_seats = 0;
        self.min_stake_for_eligibility = min_stake;
        self.lock_duration_requirement_months = 6;
        
        // ========== MILESTONE 3: GOVERNANCE DEFAULTS ==========
        self.proposal_count = 0;
        self.voting_period_days = 7;
        self.quorum_threshold = 50;      // 50% of seats must vote
        self.approval_threshold = 60;    // 60% approval needed
        self.proposal_deposit = 1_000_000_000; // 1 SOL
        self.governance_active = true;
    }
    
    pub fn has_available_seats(&self) -> bool {
        self.occupied_seats < self.total_seats
    }
    
    pub fn acquire_seat(&mut self) -> Result<()> {
        require!(self.has_available_seats(), crate::errors::SnakeError::NoAvailableSeats);
        self.occupied_seats += 1;
        Ok(())
    }
    
    pub fn release_seat(&mut self) {
        if self.occupied_seats > 0 {
            self.occupied_seats -= 1;
        }
    }
    
    // ========== MILESTONE 3: GOVERNANCE METHODS ==========
    
    pub fn next_proposal_id(&mut self) -> u64 {
        self.proposal_count += 1;
        self.proposal_count
    }
    
    pub fn calculate_quorum_required(&self) -> u32 {
        (self.occupied_seats * self.quorum_threshold) / 100
    }
    
    pub fn is_governance_active(&self) -> bool {
        self.governance_active && self.occupied_seats > 0
    }
    
    pub fn update_governance_params(
        &mut self,
        voting_period_days: Option<u8>,
        quorum_threshold: Option<u32>,
        approval_threshold: Option<u32>,
        proposal_deposit: Option<u64>,
    ) {
        if let Some(period) = voting_period_days {
            self.voting_period_days = period;
        }
        if let Some(quorum) = quorum_threshold {
            self.quorum_threshold = quorum;
        }
        if let Some(approval) = approval_threshold {
            self.approval_threshold = approval;
        }
        if let Some(deposit) = proposal_deposit {
            self.proposal_deposit = deposit;
        }
    }
}

impl Proposal {
    pub fn init(
        &mut self,
        id: u64,
        proposer: Pubkey,
        title: String,
        description: String,
        proposal_type: ProposalType,
        current_time: i64,
        voting_period_days: u8,
        bump: u8,
    ) {
        self.id = id;
        self.proposer = proposer;
        self.title = title;
        self.description = description;
        self.proposal_type = proposal_type;
        self.status = ProposalStatus::Active;
        self.created_at = current_time;
        self.voting_ends_at = current_time + (voting_period_days as i64 * 24 * 60 * 60);
        self.executed_at = 0;
        
        self.votes_for = 0;
        self.votes_against = 0;
        self.total_votes = 0;
        self.quorum_reached = false;
        
        self.target_account = None;
        self.amount = None;
        self.new_value = None;
        self.execution_data = Vec::new();
        
        self.bump = bump;
    }
    
    pub fn is_voting_active(&self, current_time: i64) -> bool {
        self.status == ProposalStatus::Active && current_time <= self.voting_ends_at
    }
    
    pub fn is_voting_ended(&self, current_time: i64) -> bool {
        current_time > self.voting_ends_at
    }
    
    pub fn add_vote(&mut self, vote_for: bool, voting_power: u32) {
        if vote_for {
            self.votes_for += voting_power;
        } else {
            self.votes_against += voting_power;
        }
        self.total_votes += voting_power;
    }
    
    pub fn finalize_voting(&mut self, dao_registry: &DaoRegistry) {
        let quorum_required = dao_registry.calculate_quorum_required();
        self.quorum_reached = self.total_votes >= quorum_required;
        
        if self.quorum_reached {
            let approval_percentage = (self.votes_for * 100) / self.total_votes;
            if approval_percentage >= dao_registry.approval_threshold {
                self.status = ProposalStatus::Passed;
            } else {
                self.status = ProposalStatus::Rejected;
            }
        } else {
            self.status = ProposalStatus::Rejected;
        }
    }
    
    pub fn execute(&mut self, current_time: i64) {
        self.status = ProposalStatus::Executed;
        self.executed_at = current_time;
    }
    
    pub fn cancel(&mut self) {
        self.status = ProposalStatus::Cancelled;
    }
}

impl Vote {
    pub fn init(
        &mut self,
        proposal_id: u64,
        voter: Pubkey,
        vote_for: bool,
        voting_power: u32,
        current_time: i64,
        bump: u8,
    ) {
        self.proposal_id = proposal_id;
        self.voter = voter;
        self.vote_for = vote_for;
        self.voting_power = voting_power;
        self.voted_at = current_time;
        self.bump = bump;
    }
}