use anchor_lang::prelude::*;

#[error_code]
pub enum SnakeError {
    #[msg("[SNAKE:6000] Unauthorized owner key")]
    Unauthorized,
    #[msg("[SNAKE:6001] Cooldown period not passed")]
    CooldownNotPassed,
    #[msg("[SNAKE:6002] Insufficient funds in treasury")]
    InsufficientFundsInTreasury,
    #[msg("[SNAKE:6003] Insufficient funds")]
    InsufficientFunds,
    #[msg("[SNAKE:6004] Invalid treasury authority")]
    InvalidTreasuryAuthority,
    #[msg("[SNAKE:6005] Arithmetic overflow occurred")]
    ArithmeticOverflow,
    #[msg("[SNAKE:6006] Claim is ended")]
    EndedClaim,

    // Patron-related errors
    #[msg("[SNAKE:6007] Patron application already exists")]
    PatronApplicationExists,
    #[msg("[SNAKE:6008] Patron not approved")]
    PatronNotApproved,
    #[msg("[SNAKE:6009] Patron already approved")]
    PatronAlreadyApproved,
    #[msg("[SNAKE:6010] Cannot exit as patron - tokens are locked")]
    PatronTokensLocked,
    #[msg("[SNAKE:6011] Only approved patrons can perform this action")]
    OnlyApprovedPatrons,

    // Lock and staking errors
    #[msg("[SNAKE:6012] Tokens are currently locked")]
    TokensLocked,
    #[msg("[SNAKE:6013] No tokens locked for staking")]
    NoTokensLocked,
    #[msg("[SNAKE:6014] Lock period not completed")]
    LockPeriodNotCompleted,
    #[msg("[SNAKE:6015] Invalid lock duration")]
    InvalidLockDuration,
    #[msg("[SNAKE:6016] Cannot lock zero tokens")]
    CannotLockZeroTokens,

    // DAO errors
    #[msg("[SNAKE:6017] Not eligible for DAO membership")]
    NotEligibleForDAO,
    #[msg("[SNAKE:6018] No available DAO seats")]
    NoAvailableSeats,
    #[msg("[SNAKE:6019] Not a DAO seat holder")]
    NotDAOSeatHolder,
    #[msg("[SNAKE:6020] Insufficient stake for DAO eligibility")]
    InsufficientStakeForDAO,
    #[msg("[SNAKE:6021] Lock duration requirement not met for DAO")]
    LockDurationRequirementNotMet,

    // Transfer restrictions
    #[msg("[SNAKE:6022] Patron tokens can only be transferred to other patrons")]
    PatronTransferRestricted,
    #[msg("[SNAKE:6023] Cannot sell tokens before lock period ends")]
    CannotSellBeforeLockEnds,
    #[msg("[SNAKE:6024] Early sale detected - commitment violated")]
    EarlySaleDetected,

    // ========== MILESTONE 2: OTC SWAP ERRORS ==========
    #[msg("[SNAKE:6025] Invalid amount specified")]
    InvalidAmount,
    #[msg("[SNAKE:6026] Invalid rate specified")]
    InvalidRate,
    #[msg("[SNAKE:6027] Invalid rebate percentage")]
    InvalidRebate,
    #[msg("[SNAKE:6028] Only normal users can sell tokens")]
    OnlyNormalUsersCanSell,
    #[msg("[SNAKE:6029] Only patrons can buy in this swap")]
    OnlyPatronsCanBuy,
    #[msg("[SNAKE:6030] Swap is not active")]
    SwapInactive,
    #[msg("[SNAKE:6031] Swap already accepted")]
    SwapAlreadyAccepted,
    #[msg("[SNAKE:6032] Cannot buy your own swap")]
    CannotBuyOwnSwap,
    #[msg("[SNAKE:6033] Swap has expired")]
    SwapExpired,
    #[msg("[SNAKE:6034] Swap has not expired yet")]
    SwapNotExpired,
    #[msg("[SNAKE:6035] Invalid swap type")]
    InvalidSwapType,
    #[msg("[SNAKE:6036] Treasury fallback not allowed for this swap")]
    TreasuryFallbackNotAllowed,
    #[msg("[SNAKE:6037] Math overflow occurred")]
    MathOverflow,
    #[msg("[SNAKE:6038] Only exiters (None role) can sell in Phase 1")]
    OnlyExitersCanSell,
    #[msg("[SNAKE:6039] Tokens are still locked and cannot be sold")]
    TokensStillLocked,
    #[msg("[SNAKE:6040] Patron has already been marked as exited")]
    PatronAlreadyExited,
    #[msg("[SNAKE:6041] Listing is not active yet (cooldown period)")]
    ListingNotActive,
    #[msg("[SNAKE:6042] Maximum OTC limit exceeded")]
    MaxOTCLimitExceeded,
    #[msg("[SNAKE:6043] Only treasury can buy in this swap")]
    OnlyTreasuryCanBuy,

    // ========== MILESTONE 3: GOVERNANCE ERRORS ==========
    #[msg("[SNAKE:6044] Governance is not active")]
    GovernanceNotActive,
    #[msg("[SNAKE:6045] Invalid proposal title")]
    InvalidProposalTitle,
    #[msg("[SNAKE:6046] Invalid proposal description")]
    InvalidProposalDescription,
    #[msg("[SNAKE:6047] Invalid proposal")]
    InvalidProposal,
    #[msg("[SNAKE:6048] Proposal is not active")]
    ProposalNotActive,
    #[msg("[SNAKE:6043] Proposal has not passed")]
    ProposalNotPassed,
    #[msg("[SNAKE:6044] Voting period has ended")]
    VotingPeriodEnded,
    #[msg("[SNAKE:6045] Voting period has not ended")]
    VotingPeriodNotEnded,
    #[msg("[SNAKE:6046] Cannot cancel proposal with votes")]
    CannotCancelProposal,
    #[msg("[SNAKE:6047] Already voted on this proposal")]
    AlreadyVoted,

    // Role transition errors
    #[msg("[SNAKE:6048] Invalid role transition")]
    InvalidRoleTransition,
    #[msg("[SNAKE:6049] Insufficient qualification score for patron")]
    InsufficientQualificationScore,
    #[msg("[SNAKE:6050] No mining history - user must mine tokens in Phase 1")]
    NoMiningHistory,

    // ========== VESTING ERRORS ==========
    #[msg("[SNAKE:6051] Vesting not unlocked yet")]
    VestingNotUnlocked,
    #[msg("[SNAKE:6052] Vesting already withdrawn")]
    VestingAlreadyWithdrawn,

    // ========== ENHANCED OTC SWAP ERRORS ==========
    #[msg("[SNAKE:6053] Swap is not active")]
    SwapNotActive,
    #[msg("[SNAKE:6054] Not whitelisted for this swap")]
    NotWhitelistedBuyer,
    #[msg("[SNAKE:6055] Insufficient role for this swap")]
    InsufficientRole,

    // ========== STUB OTC SWAP/BURN ERRORS ==========
    #[msg("[SNAKE:6056] Only patrons can perform this action")]
    OnlyPatrons,
    #[msg("[SNAKE:6057] No exit to track")]
    NoExitToTrack,

    // ========== NEW PATRON FRAMEWORK ERRORS ==========
    #[msg("[SNAKE:6058] Not eligible for OTC trading")]
    NotEligibleForOTC,
    #[msg("[SNAKE:6059] Order is for patrons only")]
    PatronOnlyOrder,
    #[msg("[SNAKE:6060] Order is for treasury only")]
    TreasuryOnlyOrder,
    #[msg("[SNAKE:6061] Insufficient patron score")]
    InsufficientPatronScore,
    #[msg("[SNAKE:6062] Order is not active")]
    OrderNotActive,
    #[msg("[SNAKE:6063] Invalid role for this action")]
    InvalidRole,
    #[msg("[SNAKE:6064] Not approved patron")]
    NotApprovedPatron,
    #[msg("[SNAKE:6065] Patron sold early")]
    PatronSoldEarly,
    #[msg("[SNAKE:6066] Vesting is not active")]
    VestingNotActive,
    #[msg("[SNAKE:6067] Nothing to claim")]
    NothingToClaim,
    #[msg("[SNAKE:6068] Month 6 milestone not reached")]
    Month6NotReached,
    #[msg("[SNAKE:6069] Maximum DAO seats reached")]
    MaxSeatsReached,
    #[msg("[SNAKE:6070] DAO seat is not active")]
    SeatNotActive,
    #[msg("[SNAKE:6071] Failed to fetch DAO seats")]
    FailedToFetchSeats,
    
    // Patron eligibility errors
    #[msg("[SNAKE:6072] Insufficient staking history")]
    InsufficientStakingHistory,
    #[msg("[SNAKE:6073] Patron eligibility criteria not met")]
    PatronEligibilityNotMet,
}
