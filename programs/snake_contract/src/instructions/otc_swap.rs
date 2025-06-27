use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Approve};
use crate::state::{UserClaim, UserRole};

#[account]
pub struct OtcSwap {
    pub exiter: Pubkey,
    pub patron: Option<Pubkey>,
    pub token_amount: u64,
    pub sol_rate: u64,           // SOL per token (in lamports)
    pub patron_rebate: u64,      // Rebate percentage (basis points, e.g., 500 = 5%)
    pub is_active: bool,
    pub created_at: i64,
    pub bump: u8,
}

#[derive(Accounts)]
#[instruction(token_amount: u64)] // Add instruction macro to access parameter
pub struct InitiateOtcSwap<'info> {
    #[account(mut)]
    pub exiter: Signer<'info>,
    
    // Verify exiter has tokens to sell (has claimed with role)
    #[account(
        constraint = exiter_claim.initialized,
        constraint = exiter_claim.role != UserRole::None,
        seeds = [b"user_claim", exiter.key().as_ref()],
        bump,
    )]
    pub exiter_claim: Account<'info, UserClaim>,
    
    #[account(
        init,
        payer = exiter,
        space = 8 + 32 + 33 + 8 + 8 + 8 + 1 + 8 + 1, // Updated space
        seeds = [b"otc_swap", exiter.key().as_ref()],
        bump,
    )]
    pub otc_swap: Account<'info, OtcSwap>,
    
    #[account(
        mut,
        constraint = exiter_token_account.owner == exiter.key(),
        constraint = exiter_token_account.amount >= token_amount,
    )]
    pub exiter_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn initiate_otc_swap(
    ctx: Context<InitiateOtcSwap>, 
    token_amount: u64, 
    sol_rate: u64,
    patron_rebate: u64
) -> Result<()> {
    require!(token_amount > 0, CustomError::InvalidAmount);
    require!(sol_rate > 0, CustomError::InvalidRate);
    require!(patron_rebate <= 1000, CustomError::InvalidRebate); // Max 10% rebate
    
    let swap = &mut ctx.accounts.otc_swap;
    swap.exiter = ctx.accounts.exiter.key();
    swap.patron = None;
    swap.token_amount = token_amount;
    swap.sol_rate = sol_rate;
    swap.patron_rebate = patron_rebate;
    swap.is_active = true;
    swap.created_at = Clock::get()?.unix_timestamp;
    swap.bump = ctx.bumps.otc_swap;
    
    // Approve the OTC swap PDA to transfer tokens
    let cpi_accounts = Approve {
        to: ctx.accounts.exiter_token_account.to_account_info(),
        delegate: ctx.accounts.otc_swap.to_account_info(),
        authority: ctx.accounts.exiter.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::approve(cpi_ctx, token_amount)?;
    
    emit!(SwapInitiated {
        exiter: ctx.accounts.exiter.key(),
        otc_swap: ctx.accounts.otc_swap.key(),
        token_amount,
        sol_rate,
        patron_rebate,
    });
    
    Ok(())
}

#[derive(Accounts)]
pub struct AcceptOtcSwap<'info> {
    #[account(mut)]
    pub patron: Signer<'info>,
    
    // Verify patron has Patron role
    #[account(
        constraint = patron_claim.initialized,
        constraint = patron_claim.role == UserRole::Patron,
        seeds = [b"user_claim", patron.key().as_ref()],
        bump,
    )]
    pub patron_claim: Account<'info, UserClaim>,
    
    /// CHECK: Validated through PDA seeds and constraints
    #[account(mut)]
    pub exiter: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"otc_swap", exiter.key().as_ref()],
        bump = otc_swap.bump,
        constraint = otc_swap.exiter == exiter.key(),
        constraint = otc_swap.is_active,
    )]
    pub otc_swap: Account<'info, OtcSwap>,
    
    #[account(
        mut,
        constraint = exiter_token_account.owner == exiter.key(),
    )]
    pub exiter_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = patron_token_account.owner == patron.key(),
    )]
    pub patron_token_account: Account<'info, TokenAccount>,
    
    // Treasury collects fees/rebates
    #[account(
        mut,
        seeds = [b"treasury"],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn accept_otc_swap(ctx: Context<AcceptOtcSwap>) -> Result<()> {
    // Extract all values we need BEFORE taking mutable borrow
    let token_amount = ctx.accounts.otc_swap.token_amount;
    let sol_rate = ctx.accounts.otc_swap.sol_rate;
    let patron_rebate = ctx.accounts.otc_swap.patron_rebate;
    let is_active = ctx.accounts.otc_swap.is_active;
    let patron_option = ctx.accounts.otc_swap.patron;
    let otc_swap_key = ctx.accounts.otc_swap.key();
    let exiter_key = ctx.accounts.exiter.key();
    let patron_key = ctx.accounts.patron.key();
    let bump = ctx.accounts.otc_swap.bump;
    
    require!(is_active, CustomError::SwapInactive);
    require!(patron_option.is_none(), CustomError::SwapAlreadyAccepted);
    
    // Calculate total payment and rebate
    let total_sol_payment = token_amount
        .checked_mul(sol_rate)
        .ok_or(CustomError::MathOverflow)?;
    
    let rebate_amount = total_sol_payment
        .checked_mul(patron_rebate)
        .ok_or(CustomError::MathOverflow)?
        .checked_div(10000) // basis points
        .ok_or(CustomError::MathOverflow)?;
    
    let net_payment = total_sol_payment
        .checked_sub(rebate_amount)
        .ok_or(CustomError::MathOverflow)?;
    
    // Verify patron has sufficient SOL
    require!(
        ctx.accounts.patron.to_account_info().lamports() >= total_sol_payment,
        CustomError::InsufficientFunds
    );
    
    // NOW take mutable borrow and update swap state
    let swap = &mut ctx.accounts.otc_swap;
    swap.patron = Some(patron_key);
    swap.is_active = false;
    
    // Create PDA signer seeds
    let signer_seeds = &[
        b"otc_swap",
        exiter_key.as_ref(),
        &[bump],
    ];
    let signer = &[&signer_seeds[..]];
    
    // Transfer tokens from exiter to patron
    let cpi_accounts = Transfer {
        from: ctx.accounts.exiter_token_account.to_account_info(),
        to: ctx.accounts.patron_token_account.to_account_info(),
        authority: ctx.accounts.otc_swap.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(), 
        cpi_accounts,
        signer,
    );
    
    token::transfer(cpi_ctx, token_amount)?;
    
    // Transfer SOL: patron pays exiter (minus rebate)
    **ctx.accounts.patron.to_account_info().try_borrow_mut_lamports()? -= total_sol_payment;
    **ctx.accounts.exiter.to_account_info().try_borrow_mut_lamports()? += net_payment;
    
    // Transfer rebate to treasury (or back to patron as reward)
    if rebate_amount > 0 {
        **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? += rebate_amount;
    }
    
    emit!(SwapCompleted {
        exiter: exiter_key,
        patron: patron_key,
        otc_swap: otc_swap_key,
        token_amount,
        sol_payment: net_payment,
        rebate_amount,
    });
    
    Ok(())
}

#[derive(Accounts)]
pub struct CancelOtcSwap<'info> {
    #[account(mut)]
    pub exiter: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"otc_swap", exiter.key().as_ref()],
        bump = otc_swap.bump,
        constraint = otc_swap.exiter == exiter.key(),
        constraint = otc_swap.is_active,
        close = exiter, // Return rent to exiter
    )]
    pub otc_swap: Account<'info, OtcSwap>,
    
    #[account(mut)]
    pub exiter_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

pub fn cancel_otc_swap(ctx: Context<CancelOtcSwap>) -> Result<()> {
    // Revoke token approval
    let cpi_accounts = anchor_spl::token::Revoke {
        source: ctx.accounts.exiter_token_account.to_account_info(),
        authority: ctx.accounts.exiter.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    anchor_spl::token::revoke(cpi_ctx)?;
    
    emit!(SwapCancelled {
        exiter: ctx.accounts.exiter.key(),
        otc_swap: ctx.accounts.otc_swap.key(),
    });
    
    Ok(())
}

// Events for tracking ecosystem activity
#[event]
pub struct SwapInitiated {
    pub exiter: Pubkey,
    pub otc_swap: Pubkey,
    pub token_amount: u64,
    pub sol_rate: u64,
    pub patron_rebate: u64,
}

#[event]
pub struct SwapCompleted {
    pub exiter: Pubkey,
    pub patron: Pubkey,
    pub otc_swap: Pubkey,
    pub token_amount: u64,
    pub sol_payment: u64,
    pub rebate_amount: u64,
}

#[event]
pub struct SwapCancelled {
    pub exiter: Pubkey,
    pub otc_swap: Pubkey,
}

#[error_code]
pub enum CustomError {
    #[msg("Swap is not active")] 
    SwapInactive,
    #[msg("Swap already accepted")]
    SwapAlreadyAccepted,
    #[msg("Invalid token amount")]
    InvalidAmount,
    #[msg("Invalid SOL rate")]
    InvalidRate,
    #[msg("Invalid rebate percentage")]
    InvalidRebate,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Only patrons can accept swaps")]
    OnlyPatrons,
}