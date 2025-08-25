use anchor_lang::prelude::*;
use crate::{
    state::{UserClaim, UserRole, PatronStatus},
    errors::SnakeError,
    utils::ValidationUtils,
    constants::*,
};
use crate::state::SwapType;

/// OTC Swap specific validation functions
pub struct OtcSwapValidation;

impl OtcSwapValidation {
    /// Validate swap parameters
    pub fn validate_swap_params(
        token_amount: u64,
        sol_rate: u64,
        buyer_rebate: u64,
    ) -> Result<()> {
        ValidationUtils::validate_amount_range(token_amount, 1, u64::MAX)?;
        ValidationUtils::validate_amount_range(sol_rate, 1, u64::MAX)?;
        ValidationUtils::validate_amount_range(buyer_rebate, 0, BASIS_POINTS)?;
        Ok(())
    }

    /// Validate seller eligibility based on swap type
    pub fn validate_seller_eligibility(
        user_claim: &UserClaim,
        swap_type: &SwapType,
        current_time: i64,
    ) -> Result<()> {
        match swap_type {
            SwapType::ExiterToPatron | SwapType::ExiterToTreasury => {
                // Phase 1: Only "None" role users (Exiters) can sell - teset by nasera
                // require!(user_claim.role == UserRole::None, SnakeError::OnlyExitersCanSell);
                
                // Ensure tokens are unlocked and eligible for sale - teset by nasera
                // require!(user_claim.can_unlock(), SnakeError::TokensStillLocked);
            },
            SwapType::PatronToPatron => {
                // Phase 2: Only active Patrons can sell to other Patrons
                ValidationUtils::validate_patron_status(user_claim)?;
                
                // Check if patron is within 6-month commitment for burn calculation
                let six_months_in_seconds = 6 * 30 * 24 * 60 * 60;
                let commitment_end = user_claim.patron_approval_timestamp + six_months_in_seconds;
                
                if current_time < commitment_end {
                    msg!("Patron exiting before 6-month commitment - 20% burn will apply");
                }
                
                // Verify patron has not already been marked as exited
                require!(!user_claim.sold_early, SnakeError::PatronAlreadyExited);
            }
        }
        
        // Ensure seller has unlocked tokens - teset by nasera
        // require!(!user_claim.is_locked(), SnakeError::TokensLocked);
        Ok(())
    }

    /// Validate buyer eligibility based on swap type
    pub fn validate_buyer_eligibility(
        buyer_claim: &UserClaim,
        swap_type: &SwapType,
    ) -> Result<()> {
        match swap_type {
            SwapType::ExiterToPatron => {
                // Phase 1: Only Patrons can buy from Exiters
                require!(buyer_claim.role == UserRole::Patron, SnakeError::OnlyPatronsCanBuy);
                require!(
                    buyer_claim.patron_status == PatronStatus::Approved,
                    SnakeError::OnlyPatronsCanBuy
                );
            },
            SwapType::ExiterToTreasury => {
                // Phase 1: Treasury fallback - no buyer validation needed
            },
            SwapType::PatronToPatron => {
                // Phase 2: Only Patrons can buy from other Patrons
                require!(buyer_claim.role == UserRole::Patron, SnakeError::OnlyPatronsCanBuy);
                require!(
                    buyer_claim.patron_status == PatronStatus::Approved,
                    SnakeError::OnlyPatronsCanBuy
                );
            }
        }
        Ok(())
    }

    /// Validate swap is active and can be executed
    pub fn validate_swap_active(
        is_active: bool,
        is_completed: bool,
        is_cancelled: bool,
        expiration_timestamp: Option<i64>,
    ) -> Result<()> {
        require!(is_active, SnakeError::SwapInactive);
        require!(!is_completed, SnakeError::SwapAlreadyAccepted);
        require!(!is_cancelled, SnakeError::SwapInactive);
        
        if let Some(expiration) = expiration_timestamp {
            let current_time = Clock::get()?.unix_timestamp;
            require!(current_time < expiration, SnakeError::SwapExpired);
        }
        
        Ok(())
    }

    /// Validate buyer has sufficient funds
    pub fn validate_buyer_funds(
        buyer_token_account_amount: u64,
        required_amount: u64,
    ) -> Result<()> {
        ValidationUtils::validate_amount_range(
            buyer_token_account_amount,
            required_amount,
            u64::MAX,
        )?;
        Ok(())
    }

    /// Validate patron exit tracking (from otc_swap_burn_stub.rs)
    pub fn validate_patron_exit_tracking(
        user_claim: &UserClaim,
        swap_amount: u64,
        current_time: i64,
    ) -> Result<()> {
        if user_claim.role == UserRole::Patron {
            // Check if patron is within 6-month commitment period
            let six_months_in_seconds = 6 * 30 * 24 * 60 * 60;
            let commitment_end = user_claim.patron_approval_timestamp + six_months_in_seconds;
            
            if current_time < commitment_end {
                msg!("Warning: Patron exiting before 6-month commitment - burn penalty may apply");
            }
            
            // Check if patron has already been marked as exited
            require!(!user_claim.sold_early, SnakeError::PatronAlreadyExited);
        }
        Ok(())
    }

    /// Validate OTC order restrictions (from otc_trading.rs)
    pub fn validate_buyer_restrictions(
        buyer_claim: &UserClaim,
        required_role: &UserRole,
    ) -> Result<()> {
        match required_role {
            UserRole::Patron => {
                require!(
                    buyer_claim.role == UserRole::Patron,
                    SnakeError::OnlyPatronsCanBuy
                );
            },
            UserRole::Staker => {
                require!(
                    buyer_claim.role == UserRole::Staker,
                    SnakeError::Unauthorized
                );
            },
            UserRole::None => {
                // No restrictions for None role
            }
        }
        Ok(())
    }
}
