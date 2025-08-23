use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::{
    state::{UserClaim, RewardPool, UserStakingHistory, GlobalStakingStats},
    constants::*,
};

/// Common account structures used across multiple instructions
pub struct CommonAccounts;

impl CommonAccounts {
    /// Standard user claim account validation
    pub fn validate_user_claim_account<'info>(
        user_claim: &Account<'info, UserClaim>,
        user: &Signer<'info>,
    ) -> Result<()> {
        require!(user_claim.initialized, crate::errors::SnakeError::Unauthorized);
        require!(user_claim.user == user.key(), crate::errors::SnakeError::Unauthorized);
        Ok(())
    }

    /// Standard token account validation
    pub fn validate_token_account<'info>(
        token_account: &Account<'info, TokenAccount>,
        owner: &Signer<'info>,
        min_amount: Option<u64>,
    ) -> Result<()> {
        require!(token_account.owner == owner.key(), crate::errors::SnakeError::Unauthorized);
        if let Some(min_amount) = min_amount {
            require!(token_account.amount >= min_amount, crate::errors::SnakeError::InsufficientFunds);
        }
        Ok(())
    }
}

/// Common account validation macro
#[macro_export]
macro_rules! validate_accounts {
    ($user_claim:expr, $user:expr) => {
        CommonAccounts::validate_user_claim_account($user_claim, $user)?
    };
    ($token_account:expr, $owner:expr, $min_amount:expr) => {
        CommonAccounts::validate_token_account($token_account, $owner, Some($min_amount))?
    };
    ($token_account:expr, $owner:expr) => {
        CommonAccounts::validate_token_account($token_account, $owner, None)?
    };
}
