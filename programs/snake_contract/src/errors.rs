use anchor_lang::prelude::*;

#[error_code]
pub enum SnakeError {
    #[msg("Unauthorized owner key")]
    Unauthorized,
    #[msg("Cooldown period not passed")]
    CooldownNotPassed,
    #[msg("Insufficient funds in treasury")]
    InsufficientFundsInTreasury,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Invalid treasury authority")]
    InvalidTreasuryAuthority,
    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    #[msg("Claim is ended")]
    EndedClaim,
    
    // Patron-related errors
    #[msg("Patron application already exists")]
    PatronApplicationExists,
    #[msg("Patron not approved")]
    PatronNotApproved,
    #[msg("Patron already approved")]
    PatronAlreadyApproved,
    #[msg("Cannot exit as patron - tokens are locked")]
    PatronTokensLocked,
    #[msg("Only approved patrons can perform this action")]
    OnlyApprovedPatrons,
    
    // Lock and staking errors
    #[msg("Tokens are currently locked")]
    TokensLocked,
    #[msg("No tokens locked for staking")]
    NoTokensLocked,
    #[msg("Lock period not completed")]
    LockPeriodNotCompleted,
    #[msg("Invalid lock duration")]
    InvalidLockDuration,
    #[msg("Cannot lock zero tokens")]
    CannotLockZeroTokens,
    
    // DAO errors
    #[msg("Not eligible for DAO membership")]
    NotEligibleForDAO,
    #[msg("No available DAO seats")]
    NoAvailableSeats,
    #[msg("Not a DAO seat holder")]
    NotDAOSeatHolder,
    #[msg("Insufficient stake for DAO eligibility")]
    InsufficientStakeForDAO,
    #[msg("Lock duration requirement not met for DAO")]
    LockDurationRequirementNotMet,
    
    // Transfer restrictions
    #[msg("Patron tokens can only be transferred to other patrons")]
    PatronTransferRestricted,
    #[msg("Cannot sell tokens before lock period ends")]
    CannotSellBeforeLockEnds,
    #[msg("Early sale detected - commitment violated")]
    EarlySaleDetected,
    
    // ========== MILESTONE 2: OTC SWAP ERRORS ==========
    
    // OTC Swap errors
    #[msg("Invalid amount specified")]
    InvalidAmount,
    #[msg("Invalid rate specified")]
    InvalidRate,
    #[msg("Invalid rebate percentage")]
    InvalidRebate,
    #[msg("Only normal users can sell tokens")]
    OnlyNormalUsersCanSell,
    #[msg("Only patrons can buy in this swap")]
    OnlyPatronsCanBuy,
    #[msg("Swap is not active")]
    SwapInactive,
    #[msg("Swap already accepted")]
    SwapAlreadyAccepted,
    #[msg("Cannot buy your own swap")]
    CannotBuyOwnSwap,
    #[msg("Swap has expired")]
    SwapExpired,
    #[msg("Swap has not expired yet")]
    SwapNotExpired,
    #[msg("Invalid swap type")]
    InvalidSwapType,
    #[msg("Treasury fallback not allowed for this swap")]
    TreasuryFallbackNotAllowed,
    #[msg("Math overflow occurred")]
    MathOverflow,
    
    // ========== MILESTONE 3: GOVERNANCE ERRORS ==========
    
    // Governance errors
    #[msg("Governance is not active")]
    GovernanceNotActive,
    #[msg("Invalid proposal title")]
    InvalidProposalTitle,
    #[msg("Invalid proposal description")]
    InvalidProposalDescription,
    #[msg("Invalid proposal")]
    InvalidProposal,
    #[msg("Proposal is not active")]
    ProposalNotActive,
    #[msg("Proposal has not passed")]
    ProposalNotPassed,
    #[msg("Voting period has ended")]
    VotingPeriodEnded,
    #[msg("Voting period has not ended")]
    VotingPeriodNotEnded,
    #[msg("Cannot cancel proposal with votes")]
    CannotCancelProposal,
    #[msg("Already voted on this proposal")]
    AlreadyVoted,
    
    // Role transition errors
    #[msg("Invalid role transition")]
    InvalidRoleTransition,
    #[msg("Insufficient qualification score for patron")]
    InsufficientQualificationScore,
    #[msg("No mining history - user must mine tokens in Phase 1")]
    NoMiningHistory,
    
    // ========== VESTING ERRORS ==========
    
    #[msg("Vesting not unlocked yet")]
    VestingNotUnlocked,
    #[msg("Vesting already withdrawn")]
    VestingAlreadyWithdrawn,
    
    // ========== ENHANCED OTC SWAP ERRORS ==========
    
    #[msg("Swap is not active")]
    SwapNotActive,
    #[msg("Not whitelisted for this swap")]
    NotWhitelistedBuyer,
    #[msg("Insufficient role for this swap")]
    InsufficientRole,
    
    // ========== STUB OTC SWAP/BURN ERRORS ==========
    
    #[msg("Only patrons can perform this action")]
    OnlyPatrons,
    #[msg("No exit to track")]
    NoExitToTrack,
}
