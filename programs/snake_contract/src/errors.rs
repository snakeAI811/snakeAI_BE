use anchor_lang::prelude::*;

#[error_code]
pub enum SnakeError {
    #[msg("[SNAKE:6000] Unauthorized owner key")]
    Unauthorized,
    #[msg("[SNAKE:6001] Cooldown period not passed")]
    CooldownNotPassed,
    #[msg("[SNAKE:6002] Yield claim cooldown period not passed")]
    YieldClaimCooldownNotPassed,
    #[msg("[SNAKE:6003] Insufficient funds in treasury")]
    InsufficientFundsInTreasury,
    #[msg("[SNAKE:6004] Insufficient funds")]
    InsufficientFunds,
    #[msg("[SNAKE:6005] Invalid treasury authority")]
    InvalidTreasuryAuthority,
    #[msg("[SNAKE:6006] Arithmetic overflow occurred")]
    ArithmeticOverflow,
    #[msg("[SNAKE:6007] Claim is ended")]
    EndedClaim,

    // Patron-related errors
    #[msg("[SNAKE:6008] Patron application already exists")]
    PatronApplicationExists,
    #[msg("[SNAKE:6009] Patron not approved")]
    PatronNotApproved,
    #[msg("[SNAKE:6010] Patron already approved")]
    PatronAlreadyApproved,
    #[msg("[SNAKE:6011] Cannot exit as patron - tokens are locked")]
    PatronTokensLocked,
    #[msg("[SNAKE:6012] Only approved patrons can perform this action")]
    OnlyApprovedPatrons,

    // Lock and staking errors
    #[msg("[SNAKE:6013] Tokens are currently locked")]
    TokensLocked,
    #[msg("[SNAKE:6014] No tokens locked for staking")]
    NoTokensLocked,
    #[msg("[SNAKE:6015] Lock period not completed")]
    LockPeriodNotCompleted,
    #[msg("[SNAKE:6016] Invalid lock duration")]
    InvalidLockDuration,
    #[msg("[SNAKE:6017] Cannot lock zero tokens")]
    CannotLockZeroTokens,

    // DAO errors
    #[msg("[SNAKE:6018] Not eligible for DAO membership")]
    NotEligibleForDAO,
    #[msg("[SNAKE:6019] No available DAO seats")]
    NoAvailableSeats,
    #[msg("[SNAKE:6020] Not a DAO seat holder")]
    NotDAOSeatHolder,
    #[msg("[SNAKE:6021] Insufficient stake for DAO eligibility")]
    InsufficientStakeForDAO,
    #[msg("[SNAKE:6022] Lock duration requirement not met for DAO")]
    LockDurationRequirementNotMet,

    // Transfer restrictions
    #[msg("[SNAKE:6023] Patron tokens can only be transferred to other patrons")]
    PatronTransferRestricted,
    #[msg("[SNAKE:6024] Cannot sell tokens before lock period ends")]
    CannotSellBeforeLockEnds,
    #[msg("[SNAKE:6025] Early sale detected - commitment violated")]
    EarlySaleDetected,

    // ========== MILESTONE 2: OTC SWAP ERRORS ==========
    #[msg("[SNAKE:6026] Invalid amount specified")]
    InvalidAmount,
    #[msg("[SNAKE:6027] Invalid rate specified")]
    InvalidRate,
    #[msg("[SNAKE:6028] Invalid rebate percentage")]
    InvalidRebate,
    #[msg("[SNAKE:6029] Only normal users can sell tokens")]
    OnlyNormalUsersCanSell,
    #[msg("[SNAKE:6030] Only patrons can buy in this swap")]
    OnlyPatronsCanBuy,
    #[msg("[SNAKE:6031] Swap is not active")]
    SwapInactive,
    #[msg("[SNAKE:6032] Swap already accepted")]
    SwapAlreadyAccepted,
    #[msg("[SNAKE:6033] Cannot buy your own swap")]
    CannotBuyOwnSwap,
    #[msg("[SNAKE:6034] Swap has expired")]
    SwapExpired,
    #[msg("[SNAKE:6035] Swap has not expired yet")]
    SwapNotExpired,
    #[msg("[SNAKE:6036] Invalid swap type")]
    InvalidSwapType,
    #[msg("[SNAKE:6037] Treasury fallback not allowed for this swap")]
    TreasuryFallbackNotAllowed,
    #[msg("[SNAKE:6038] Math overflow occurred")]
    MathOverflow,
    #[msg("[SNAKE:6039] Only exiters (None role) can sell in Phase 1")]
    OnlyExitersCanSell,
    #[msg("[SNAKE:6040] Tokens are still locked and cannot be sold")]
    TokensStillLocked,
    #[msg("[SNAKE:6041] Patron has already been marked as exited")]
    PatronAlreadyExited,
    #[msg("[SNAKE:6042] Listing is not active yet (cooldown period)")]
    ListingNotActive,
    #[msg("[SNAKE:6043] Maximum OTC limit exceeded")]
    MaxOTCLimitExceeded,
    #[msg("[SNAKE:6044] Only treasury can buy in this swap")]
    OnlyTreasuryCanBuy,

    // ========== MILESTONE 3: GOVERNANCE ERRORS ==========
    #[msg("[SNAKE:6045] Governance is not active")]
    GovernanceNotActive,
    #[msg("[SNAKE:6046] Invalid proposal title")]
    InvalidProposalTitle,
    #[msg("[SNAKE:6047] Invalid proposal description")]
    InvalidProposalDescription,
    #[msg("[SNAKE:6048] Invalid proposal")]
    InvalidProposal,
    #[msg("[SNAKE:6049] Proposal is not active")]
    ProposalNotActive,
    #[msg("[SNAKE:6050] Proposal has not passed")]
    ProposalNotPassed,
    #[msg("[SNAKE:6051] Voting period has ended")]
    VotingPeriodEnded,
    #[msg("[SNAKE:6052] Voting period has not ended")]
    VotingPeriodNotEnded,
    #[msg("[SNAKE:6053] Cannot cancel proposal with votes")]
    CannotCancelProposal,
    #[msg("[SNAKE:6054] Already voted on this proposal")]
    AlreadyVoted,

    // Role transition errors
    #[msg("[SNAKE:6055] Invalid role transition")]
    InvalidRoleTransition,
    #[msg("[SNAKE:6056] Insufficient qualification score for patron")]
    InsufficientQualificationScore,
    #[msg("[SNAKE:6057] No mining history - user must mine tokens in Phase 1")]
    NoMiningHistory,

    // ========== VESTING ERRORS ==========
    #[msg("[SNAKE:6058] Vesting not unlocked yet")]
    VestingNotUnlocked,
    #[msg("[SNAKE:6059] Vesting already withdrawn")]
    VestingAlreadyWithdrawn,

    // ========== ENHANCED OTC SWAP ERRORS ==========
    #[msg("[SNAKE:6060] Swap is not active")]
    SwapNotActive,
    #[msg("[SNAKE:6061] Not whitelisted for this swap")]
    NotWhitelistedBuyer,
    #[msg("[SNAKE:6062] Insufficient role for this swap")]
    InsufficientRole,

    // ========== STUB OTC SWAP/BURN ERRORS ==========
    #[msg("[SNAKE:6063] Only patrons can perform this action")]
    OnlyPatrons,
    #[msg("[SNAKE:6064] No exit to track")]
    NoExitToTrack,

    // ========== NEW PATRON FRAMEWORK ERRORS ==========
    #[msg("[SNAKE:6065] Not eligible for OTC trading")]
    NotEligibleForOTC,
    #[msg("[SNAKE:6066] Order is for patrons only")]
    PatronOnlyOrder,
    #[msg("[SNAKE:6067] Order is for treasury only")]
    TreasuryOnlyOrder,
    #[msg("[SNAKE:6068] Insufficient patron score")]
    InsufficientPatronScore,
    #[msg("[SNAKE:6069] Order is not active")]
    OrderNotActive,
    #[msg("[SNAKE:6070] Invalid role for this action")]
    InvalidRole,
    #[msg("[SNAKE:6071] Not approved patron")]
    NotApprovedPatron,
    #[msg("[SNAKE:6072] Patron sold early")]
    PatronSoldEarly,
    #[msg("[SNAKE:6073] Vesting is not active")]
    VestingNotActive,
    #[msg("[SNAKE:6074] Nothing to claim")]
    NothingToClaim,
    #[msg("[SNAKE:6075] Month 6 milestone not reached")]
    Month6NotReached,
    #[msg("[SNAKE:6076] Maximum DAO seats reached")]
    MaxSeatsReached,
    #[msg("[SNAKE:6077] DAO seat is not active")]
    SeatNotActive,
    #[msg("[SNAKE:6078] Failed to fetch DAO seats")]
    FailedToFetchSeats,
    
    // Patron eligibility errors
    #[msg("[SNAKE:6079] Insufficient staking history")]
    InsufficientStakingHistory,
    #[msg("[SNAKE:6080] Patron eligibility criteria not met")]
    PatronEligibilityNotMet,

    // ========== NEW STAKING ERRORS ==========
    #[msg("[SNAKE:6081] Insufficient token amount for staking")]
    InsufficientTokenAmount,
    #[msg("[SNAKE:6082] Invalid user role for this action")]
    InvalidUserRole,
    #[msg("[SNAKE:6083] Invalid APY rate - must be between 0-100%")]
    InvalidAPYRate,
}