use anchor_lang::prelude::*;

#[error_code]
pub enum SnakeError {
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
}
