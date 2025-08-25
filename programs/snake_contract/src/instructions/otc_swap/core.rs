use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Approve, Burn, Mint};
use crate::{
    state::{UserClaim, OtcSwap, UserRole},
    errors::SnakeError,
    utils::{CalculationUtils},
    constants::*,
};
use super::{
    deflationary::{DeflationaryMechanics, DailyVolumeTracker},
};
use crate::state::SwapType;

/// Core OTC swap business logic
pub struct OtcSwapCore;

impl OtcSwapCore {
    /// Initialize OTC swap based on swap type
    pub fn initialize_swap(
        otc_swap: &mut OtcSwap,
        seller: Pubkey,
        token_amount: u64,
        sol_rate: u64,
        buyer_rebate: u64,
        swap_type: SwapType,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        match swap_type {
            SwapType::ExiterToPatron => {
                Self::init_exiter_to_patron_swap(
                    otc_swap,
                    seller,
                    token_amount,
                    sol_rate,
                    buyer_rebate,
                    current_time,
                    bump,
                )?;
            },
            SwapType::ExiterToTreasury => {
                Self::init_treasury_buyback_swap(
                    otc_swap,
                    seller,
                    token_amount,
                    sol_rate,
                    current_time,
                    bump,
                )?;
            },
            SwapType::PatronToPatron => {
                Self::init_patron_to_patron_swap(
                    otc_swap,
                    seller,
                    token_amount,
                    sol_rate,
                    current_time,
                    bump,
                )?;
            }
        }
        Ok(())
    }

    /// Initialize Exiter to Patron swap (Phase 1)
    fn init_exiter_to_patron_swap(
        otc_swap: &mut OtcSwap,
        seller: Pubkey,
        token_amount: u64,
        fixed_price: u64,
        buyer_rebate: u64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        let max_otc_limit = 1_000_000 * LAMPORTS_PER_SNK; // 1M tokens max per user
        
        otc_swap.init_exiter_to_patron(
            seller,
            token_amount,
            fixed_price,
            buyer_rebate,
            max_otc_limit,
            current_time,
            bump,
        );
        Ok(())
    }

    /// Initialize Treasury buyback swap (Phase 1 fallback)
    fn init_treasury_buyback_swap(
        otc_swap: &mut OtcSwap,
        seller: Pubkey,
        token_amount: u64,
        fixed_price: u64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        otc_swap.init_treasury_buyback(
            seller,
            token_amount,
            fixed_price,
            current_time,
            bump,
        );
        Ok(())
    }

    /// Initialize Patron to Patron swap (Phase 2)
    fn init_patron_to_patron_swap(
        otc_swap: &mut OtcSwap,
        seller: Pubkey,
        token_amount: u64,
        asking_price: u64,
        current_time: i64,
        bump: u8,
    ) -> Result<()> {
        let cooldown_period = 24 * 60 * 60; // 24 hours cooldown
        
        otc_swap.init_patron_to_patron(
            seller,
            token_amount,
            asking_price,
            cooldown_period,
            current_time,
            bump,
        );
        Ok(())
    }

    /// Execute token transfer for swap
    pub fn execute_token_transfer<'info>(
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
        authority: &AccountInfo<'info>,
        amount: u64,
        token_program: &Program<'info, Token>,
        signer_seeds: Option<&[&[&[u8]]]>,
    ) -> Result<()> {
        let cpi_accounts = Transfer {
            from: from.to_account_info(),
            to: to.to_account_info(),
            authority: authority.to_account_info(),
        };

        let cpi_ctx = if let Some(seeds) = signer_seeds {
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                cpi_accounts,
                seeds,
            )
        } else {
            CpiContext::new(token_program.to_account_info(), cpi_accounts)
        };

        token::transfer(cpi_ctx, amount)?;
        Ok(())
    }

    /// Execute complete OTC swap with deflationary mechanics
    pub fn execute_otc_swap_with_deflationary<'info>(
        otc_swap: &mut OtcSwap,
        seller_claim: &mut UserClaim,
        buyer_claim: &UserClaim,
        token_amount: u64,
        seller_token_account: &Account<'info, TokenAccount>,
        buyer_token_account: &Account<'info, TokenAccount>,
        treasury_account: &Account<'info, TokenAccount>,
        mint: &Account<'info, Mint>,
        authority: &AccountInfo<'info>,
        token_program: &Program<'info, Token>,
        signer_seeds: &[&[&[u8]]],
        daily_volume_tracker: &mut DailyVolumeTracker,
        current_time: i64,
    ) -> Result<()> {
        // Calculate deflationary amounts
        let burn_amount = DeflationaryMechanics::calculate_burn_amount(
            token_amount,
            &otc_swap.swap_type,
            seller_claim,
            current_time,
        );
        
        let treasury_skim = DeflationaryMechanics::calculate_treasury_skim(
            token_amount,
            &otc_swap.swap_type,
        );
        
        // Calculate net amount after deflationary operations
        let net_amount = token_amount
            .checked_sub(burn_amount)
            .ok_or(SnakeError::ArithmeticOverflow)?
            .checked_sub(treasury_skim)
            .ok_or(SnakeError::ArithmeticOverflow)?;
        
        // Update daily volume tracker
        daily_volume_tracker.add_volume(token_amount)?;
        
        // Execute token transfer (net amount to buyer)
        Self::execute_token_transfer(
            seller_token_account,
            buyer_token_account,
            authority,
            net_amount,
            token_program,
            Some(signer_seeds),
        )?;
        
        // Execute deflationary operations (burn + treasury skim)
        DeflationaryMechanics::execute_deflationary_operations(
            token_amount,
            burn_amount,
            treasury_skim,
            seller_token_account,
            treasury_account,
            mint,
            authority,
            token_program,
            signer_seeds,
        )?;
        
        // Mark seller as exited if applicable
        if seller_claim.role == UserRole::Patron {
            seller_claim.sold_early = true;
        }
        
        // Update swap status
        otc_swap.is_active = false;
        otc_swap.buyer = Some(buyer_claim.user);
        
        Ok(())
    }

    /// Calculate burn amount for patron exit penalty (legacy function)
    pub fn calculate_patron_exit_burn(
        token_amount: u64,
        swap_type: &SwapType,
        user_claim: &UserClaim,
        current_time: i64,
    ) -> u64 {
        match swap_type {
            SwapType::PatronToPatron => {
                // Check if patron is within 6-month commitment period
                let six_months_in_seconds = 6 * 30 * 24 * 60 * 60;
                let commitment_end = user_claim.patron_approval_timestamp + six_months_in_seconds;
                
                if current_time < commitment_end {
                    CalculationUtils::calculate_patron_exit_burn(token_amount)
                } else {
                    0 // No burn after commitment period
                }
            },
            _ => 0, // No burn for other swap types
        }
    }

    /// Execute token burn for patron exit penalty
    pub fn execute_token_burn<'info>(
        token_account: &Account<'info, TokenAccount>,
        mint: &Account<'info, Mint>,
        authority: &AccountInfo<'info>,
        amount: u64,
        token_program: &Program<'info, Token>,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let cpi_accounts = Burn {
            mint: mint.to_account_info(),
            from: token_account.to_account_info(),
            authority: authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        token::burn(cpi_ctx, amount)?;
        Ok(())
    }

    /// Approve tokens for OTC swap
    pub fn approve_tokens<'info>(
        token_account: &Account<'info, TokenAccount>,
        delegate: &AccountInfo<'info>,
        authority: &Signer<'info>,
        amount: u64,
        token_program: &Program<'info, Token>,
    ) -> Result<()> {
        let cpi_accounts = Approve {
            to: token_account.to_account_info(),
            delegate: delegate.to_account_info(),
            authority: authority.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(token_program.to_account_info(), cpi_accounts);
        token::approve(cpi_ctx, amount)?;
        Ok(())
    }
}
