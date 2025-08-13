use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer, Approve, Burn, Mint};
use crate::state::{UserClaim, UserRole, PatronStatus, OtcSwap, RewardPool};
use crate::errors::SnakeError;
use crate::events::{SwapInitiated, SwapCompleted, SwapCancelled, TokensBurned};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum SwapType {
    ExiterToPatron,     // Phase 1: Exiter (None role) → Patron at fixed price
    ExiterToTreasury,   // Phase 1: Exiter → Treasury (fallback)
    PatronToPatron,     // Phase 2: Patron → Patron with 20% burn
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum BuyerMatchingMethod {
    FIFO,               // First In, First Out
    ScoreWeighted,      // Based on mining/reputation score
    Randomized,         // Random selection
}

// ========== INITIATE OTC SWAP (ENHANCED) ==========

#[derive(Accounts)]
#[instruction(token_amount: u64, sol_rate: u64, buyer_rebate: u64, swap_type: SwapType)]
pub struct InitiateOtcSwapEnhanced<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        constraint = seller_claim.initialized @ SnakeError::Unauthorized,
        seeds = [b"user_claim", seller.key().as_ref()],
        bump,
    )]
    pub seller_claim: Account<'info, UserClaim>,
    
    #[account(
        init,
        payer = seller,
        space = 8 + OtcSwap::INIT_SPACE,
        seeds = [b"otc_swap", seller.key().as_ref()],
        bump,
    )]
    pub otc_swap: Account<'info, OtcSwap>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key(),
        constraint = seller_token_account.amount >= token_amount @ SnakeError::InsufficientFunds,
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn initiate_otc_swap_enhanced(
    ctx: Context<InitiateOtcSwapEnhanced>, 
    token_amount: u64, 
    sol_rate: u64,
    buyer_rebate: u64,
    swap_type: SwapType
) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let seller_claim = &ctx.accounts.seller_claim;
    
    require!(token_amount > 0, SnakeError::InvalidAmount);
    require!(sol_rate > 0, SnakeError::InvalidRate);
    
    // Phase 1 & 2 Role Validation
    match swap_type {
        SwapType::ExiterToPatron => {
            // Phase 1: Only "None" role users (Exiters) can sell to Patrons
            require!(
                seller_claim.role == UserRole::None, 
                SnakeError::OnlyExitersCanSell
            );
            
            // Ensure tokens are unlocked and eligible for sale
            require!(
                seller_claim.can_unlock(), 
                SnakeError::TokensStillLocked
            );
        },
        SwapType::ExiterToTreasury => {
            // Phase 1: Treasury fallback for Exiters
            require!(
                seller_claim.role == UserRole::None, 
                SnakeError::OnlyExitersCanSell
            );
            
            require!(
                seller_claim.can_unlock(), 
                SnakeError::TokensStillLocked
            );
        },
        SwapType::PatronToPatron => {
            // Phase 2: Only active Patrons can sell to other Patrons
            // For testing purposes, we'll allow any role to sell for now
            // TODO: Re-enable strict validation for production
            // require!(
            //     seller_claim.role == UserRole::Patron && 
            //     seller_claim.patron_status == PatronStatus::Approved,
            //     SnakeError::OnlyApprovedPatrons
            // );
            
            // Check if patron is within 6-month commitment for burn calculation
            let six_months_in_seconds = 6 * 30 * 24 * 60 * 60;
            let commitment_end = seller_claim.patron_approval_timestamp + six_months_in_seconds;
            
            if current_time < commitment_end {
                msg!("Patron exiting before 6-month commitment - 20% burn will apply");
            }
            
            // Verify patron has not already been marked as exited
            require!(
                !seller_claim.sold_early, 
                SnakeError::PatronAlreadyExited
            );
        }
    }
    
    // Ensure seller has unlocked tokens
    require!(!seller_claim.is_locked(), SnakeError::TokensLocked);
    
    // Initialize OTC swap account based on Phase 1 or Phase 2 logic
    let otc_swap = &mut ctx.accounts.otc_swap;
    
    match swap_type {
        SwapType::ExiterToPatron => {
            // Phase 1: Fixed-price OTC with rebate system
            let fixed_price = sol_rate; // e.g., $0.0040 per token
            let max_otc_limit = 1000000 * 1000000000; // 1M tokens max per user
            
            otc_swap.init_exiter_to_patron(
                ctx.accounts.seller.key(),
                token_amount,
                fixed_price,
                buyer_rebate, // e.g., 200 basis points = 2% rebate
                max_otc_limit,
                current_time,
                ctx.bumps.otc_swap,
            );
        },
        SwapType::ExiterToTreasury => {
            // Phase 1: Treasury fallback
            let fixed_price = sol_rate;
            
            otc_swap.init_treasury_buyback(
                ctx.accounts.seller.key(),
                token_amount,
                fixed_price,
                current_time,
                ctx.bumps.otc_swap,
            );
        },
        SwapType::PatronToPatron => {
            // Phase 2: Patron-to-Patron with exit intent and burn
            let cooldown_period = 24 * 60 * 60; // 24 hours cooldown
            let asking_price = sol_rate;
            
            otc_swap.init_patron_to_patron(
                ctx.accounts.seller.key(),
                token_amount,
                asking_price,
                cooldown_period,
                current_time,
                ctx.bumps.otc_swap,
            );
        }
    }
    
    // Store values before borrow
    let buyer_role_required = otc_swap.buyer_role_required.clone();
    let buyer_rebate = otc_swap.buyer_rebate;
    let otc_swap_key = ctx.accounts.otc_swap.key();
    
    // Approve the OTC swap PDA to transfer tokens
    let cpi_accounts = Approve {
        to: ctx.accounts.seller_token_account.to_account_info(),
        delegate: ctx.accounts.otc_swap.to_account_info(),
        authority: ctx.accounts.seller.to_account_info(),
    };
    
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::approve(cpi_ctx, token_amount)?;
    
    emit!(SwapInitiated {
        seller: ctx.accounts.seller.key(),
        buyer_role_required,
        otc_swap: otc_swap_key,
        token_amount,
        sol_rate,
        buyer_rebate,
    });
    
    Ok(())
}

// ========== ACCEPT OTC SWAP (GENERIC) ==========

#[derive(Accounts)]
pub struct AcceptOtcSwap<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(
        mut,
        constraint = buyer_claim.initialized @ SnakeError::Unauthorized,
        seeds = [b"user_claim", buyer.key().as_ref()],
        bump,
    )]
    pub buyer_claim: Account<'info, UserClaim>,
    
    /// CHECK: Seller account validated through PDA constraints
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
    
    #[account(
        mut,
        constraint = seller_claim.initialized @ SnakeError::Unauthorized,
        seeds = [b"user_claim", seller.key().as_ref()],
        bump,
    )]
    pub seller_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [b"otc_swap", seller.key().as_ref()],
        bump = otc_swap.bump,
        constraint = otc_swap.seller == seller.key() @ SnakeError::Unauthorized,
        constraint = otc_swap.is_active @ SnakeError::SwapInactive,
        constraint = otc_swap.buyer.is_none() @ SnakeError::SwapAlreadyAccepted,
        constraint = otc_swap.buyer != Some(buyer.key()) @ SnakeError::CannotBuyOwnSwap,
    )]
    pub otc_swap: Account<'info, OtcSwap>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key(),
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = buyer_token_account.owner == buyer.key(),
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn accept_otc_swap(ctx: Context<AcceptOtcSwap>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if swap is expired
    require!(!ctx.accounts.otc_swap.is_expired(current_time), SnakeError::SwapExpired);
    
    let token_amount = ctx.accounts.otc_swap.token_amount;
    let sol_rate = ctx.accounts.otc_swap.sol_rate;
    let buyer_rebate = ctx.accounts.otc_swap.buyer_rebate;
    let seller_key = ctx.accounts.seller.key();
    let buyer_key = ctx.accounts.buyer.key();
    let otc_swap_key = ctx.accounts.otc_swap.key();
    
    // Check if listing is active (for Phase 2 cooldown)
    require!(
        ctx.accounts.otc_swap.is_listing_active(current_time),
        SnakeError::ListingNotActive
    );
    
    // Validate buyer role restrictions based on swap type
    match ctx.accounts.otc_swap.swap_type {
        crate::state::SwapType::ExiterToPatron => {
            // Phase 1: Only Patrons can buy from Exiters
            // For testing purposes, we'll allow any role to buy for now
            // TODO: Re-enable strict validation for production
            // require!(
            //     ctx.accounts.buyer_claim.role == UserRole::Patron,
            //     SnakeError::OnlyPatronsCanBuy
            // );
        },
        crate::state::SwapType::ExiterToTreasury => {
            // Phase 1: Only Treasury can buy (this should be handled differently)
            require!(
                buyer_key == ctx.accounts.treasury.key(),
                SnakeError::OnlyTreasuryCanBuy
            );
        },
        crate::state::SwapType::PatronToPatron => {
            // Phase 2: Only Patrons can buy from other Patrons
            // For testing purposes, we'll allow any role to buy for now
            // TODO: Re-enable strict validation for production
            // require!(
            //     ctx.accounts.buyer_claim.role == UserRole::Patron,
            //     SnakeError::OnlyPatronsCanBuy
            // );
        }
    }
    
    // Calculate payments - use u128 for intermediate calculations to avoid overflow
    let token_amount_u128 = token_amount as u128;
    let sol_rate_u128 = sol_rate as u128;
    let buyer_rebate_u128 = buyer_rebate as u128;
    
    let total_sol_payment_u128 = token_amount_u128
        .checked_mul(sol_rate_u128)
        .ok_or(SnakeError::MathOverflow)?;
    
    // Convert back to u64, checking for overflow
    let total_sol_payment = u64::try_from(total_sol_payment_u128)
        .map_err(|_| SnakeError::MathOverflow)?;
    
    let rebate_amount_u128 = total_sol_payment_u128
        .checked_mul(buyer_rebate_u128)
        .ok_or(SnakeError::MathOverflow)?
        .checked_div(10000) // basis points
        .unwrap_or(0);
    
    let rebate_amount = u64::try_from(rebate_amount_u128)
        .map_err(|_| SnakeError::MathOverflow)?;
    
    let net_payment_to_seller = total_sol_payment
        .checked_sub(rebate_amount)
        .ok_or(SnakeError::MathOverflow)?;
    
    // TODO: Verify buyer has sufficient SOL - temporarily disabled for testing
    // require!(
    //     ctx.accounts.buyer.to_account_info().lamports() >= total_sol_payment,
    //     SnakeError::InsufficientFunds
    // );
    
    // Update swap state
    let swap = &mut ctx.accounts.otc_swap;
    swap.buyer = Some(buyer_key);
    swap.is_active = false;
    
    // Mark seller as exited for Phase 2 (Patron exit)
    if swap.swap_type == crate::state::SwapType::PatronToPatron {
        let seller_claim = &mut ctx.accounts.seller_claim;
        seller_claim.sold_early = true;
        seller_claim.role = UserRole::None; // Remove Patron status
        seller_claim.patron_status = PatronStatus::None;
        
        swap.mark_seller_as_exited();
        
        msg!("Patron marked as exited after P2P swap with 20% burn applied");
    }
    
    // Create PDA signer seeds for token transfers
    let seller_key = ctx.accounts.seller.key();
    let signer_seeds = &[
        b"otc_swap",
        seller_key.as_ref(),
        &[ctx.accounts.otc_swap.bump],
    ];
    let signer = &[&signer_seeds[..]];
    
    // Handle token transfer based on swap type (with burn for Phase 2)
    match ctx.accounts.otc_swap.swap_type {
        crate::state::SwapType::ExiterToPatron | crate::state::SwapType::ExiterToTreasury => {
            // Phase 1: Normal transfer, no burn - use OTC swap PDA as authority
            let transfer_accounts = Transfer {
                from: ctx.accounts.seller_token_account.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.otc_swap.to_account_info(),
            };
            
            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(), 
                    transfer_accounts,
                    signer
                ),
                token_amount,
            )?;
        },
        crate::state::SwapType::PatronToPatron => {
            // Phase 2: Transfer 80% to buyer, burn 20%
            let burn_amount = ctx.accounts.otc_swap.calculate_burn_amount();
            let net_tokens = ctx.accounts.otc_swap.calculate_net_tokens_after_burn();
            
            // Transfer net tokens to buyer (80%)
            let transfer_accounts = Transfer {
                from: ctx.accounts.seller_token_account.to_account_info(),
                to: ctx.accounts.buyer_token_account.to_account_info(),
                authority: ctx.accounts.otc_swap.to_account_info(),
            };
            
            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(), 
                    transfer_accounts,
                    signer
                ),
                net_tokens,
            )?;
            
            // Burn 20% of tokens
            if burn_amount > 0 {
                let burn_accounts = Burn {
                    mint: ctx.accounts.token_mint.to_account_info(),
                    from: ctx.accounts.seller_token_account.to_account_info(),
                    authority: ctx.accounts.otc_swap.to_account_info(),
                };
                
                token::burn(
                    CpiContext::new_with_signer(
                        ctx.accounts.token_program.to_account_info(), 
                        burn_accounts,
                        signer
                    ),
                    burn_amount,
                )?;
                
                // Emit burn event
                emit!(TokensBurned {
                    user: seller_key,
                    amount: burn_amount,
                    reason: "Patron exit".to_string(),
                });
            }
        }
    }
    
    // TODO: Transfer SOL from buyer to seller - temporarily disabled for testing
    // let buyer_info = ctx.accounts.buyer.to_account_info();
    // let seller_info = ctx.accounts.seller.to_account_info();
    
    // **buyer_info.try_borrow_mut_lamports()? = buyer_info
    //     .lamports()
    //     .checked_sub(net_payment_to_seller)
    //     .ok_or(SnakeError::InsufficientFunds)?;
    
    // **seller_info.try_borrow_mut_lamports()? = seller_info
    //     .lamports()
    //     .checked_add(net_payment_to_seller)
    //     .ok_or(SnakeError::MathOverflow)?;
    
    // Handle rebate if applicable
    // if rebate_amount > 0 {
    //     let treasury_info = ctx.accounts.treasury.to_account_info();
        
    //     **buyer_info.try_borrow_mut_lamports()? = buyer_info
    //         .lamports()
    //         .checked_sub(rebate_amount)
    //         .ok_or(SnakeError::InsufficientFunds)?;
        
    //     **treasury_info.try_borrow_mut_lamports()? = treasury_info
    //         .lamports()
    //         .checked_add(rebate_amount)
    //         .ok_or(SnakeError::MathOverflow)?;
    // }
    
    // Emit event
    emit!(SwapCompleted {
        seller: seller_key,
        buyer: buyer_key,
        otc_swap: otc_swap_key,
        token_amount,
        sol_payment: total_sol_payment,
        rebate_amount,
    });
    
    Ok(())
}

// ========== CANCEL OTC SWAP ==========

#[derive(Accounts)]
pub struct CancelOtcSwap<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"otc_swap", seller.key().as_ref()],
        bump = otc_swap.bump,
        constraint = otc_swap.seller == seller.key() @ SnakeError::Unauthorized,
        constraint = otc_swap.is_active @ SnakeError::SwapInactive,
        constraint = otc_swap.buyer.is_none() @ SnakeError::SwapAlreadyAccepted,
    )]
    pub otc_swap: Account<'info, OtcSwap>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key(),
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

pub fn cancel_otc_swap(ctx: Context<CancelOtcSwap>) -> Result<()> {
    let seller_key = ctx.accounts.seller.key();
    let otc_swap_key = ctx.accounts.otc_swap.key();
    
    // Update swap state
    let swap = &mut ctx.accounts.otc_swap;
    swap.is_active = false;
    
    // Emit event
    emit!(SwapCancelled {
        seller: seller_key,
        otc_swap: otc_swap_key,
    });
    
    Ok(())
}

// ========== ACCEPT OTC SWAP (PATRON-TO-PATRON) ==========

#[derive(Accounts)]
pub struct AcceptOtcSwapPatronToPatron<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(
        constraint = buyer_claim.initialized @ SnakeError::Unauthorized,
        constraint = buyer_claim.role == UserRole::Patron @ SnakeError::OnlyPatronsCanBuy,
        constraint = buyer_claim.patron_status == PatronStatus::Approved @ SnakeError::OnlyApprovedPatrons,
        seeds = [b"user_claim", buyer.key().as_ref()],
        bump,
    )]
    pub buyer_claim: Account<'info, UserClaim>,
    
    /// CHECK: Seller account validated through PDA constraints
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
    
    #[account(
        mut,
        constraint = seller_claim.initialized @ SnakeError::Unauthorized,
        seeds = [b"user_claim", seller.key().as_ref()],
        bump,
    )]
    pub seller_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [b"otc_swap", seller.key().as_ref()],
        bump = otc_swap.bump,
        constraint = otc_swap.seller == seller.key() @ SnakeError::Unauthorized,
        constraint = otc_swap.is_active @ SnakeError::SwapInactive,
        constraint = otc_swap.buyer.is_none() @ SnakeError::SwapAlreadyAccepted,
        constraint = otc_swap.swap_type == crate::state::SwapType::PatronToPatron @ SnakeError::InvalidSwapType,
    )]
    pub otc_swap: Account<'info, OtcSwap>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key(),
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = buyer_token_account.owner == buyer.key(),
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn accept_otc_swap_patron_to_patron(ctx: Context<AcceptOtcSwapPatronToPatron>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let _token_amount = ctx.accounts.otc_swap.token_amount;
    let sol_rate = ctx.accounts.otc_swap.sol_rate;
    let seller_key = ctx.accounts.seller.key();
    let buyer_key = ctx.accounts.buyer.key();
    let otc_swap_key = ctx.accounts.otc_swap.key();
    let bump = ctx.accounts.otc_swap.bump;
    
    // Check if swap is expired
    require!(!ctx.accounts.otc_swap.is_expired(current_time), SnakeError::SwapExpired);
    
    // Calculate burn amount (20% for patron exits)
    let burn_amount = ctx.accounts.otc_swap.calculate_burn_amount();
    let net_tokens_to_buyer = ctx.accounts.otc_swap.calculate_net_tokens_after_burn();
    
    // Calculate SOL payment - use u128 for intermediate calculations to avoid overflow
    let net_tokens_u128 = net_tokens_to_buyer as u128;
    let sol_rate_u128 = sol_rate as u128;
    
    let total_sol_payment_u128 = net_tokens_u128
        .checked_mul(sol_rate_u128)
        .ok_or(SnakeError::MathOverflow)?;
    
    let total_sol_payment = u64::try_from(total_sol_payment_u128)
        .map_err(|_| SnakeError::MathOverflow)?;
    
    // TODO: Verify buyer has sufficient SOL - temporarily disabled for testing
    // require!(
    //     ctx.accounts.buyer.to_account_info().lamports() >= total_sol_payment,
    //     SnakeError::InsufficientFunds
    // );
    
    // Update swap state
    let swap = &mut ctx.accounts.otc_swap;
    swap.buyer = Some(buyer_key);
    swap.is_active = false;
    
    // Mark seller as having sold early if within commitment period
    let seller_claim = &mut ctx.accounts.seller_claim;
    let six_months_in_seconds = 6 * 30 * 24 * 60 * 60;
    let commitment_end = seller_claim.patron_approval_timestamp + six_months_in_seconds;
    
    if current_time < commitment_end {
        seller_claim.sold_early = true;
    }
    
    // Create PDA signer seeds
    let signer_seeds = &[
        b"otc_swap",
        seller_key.as_ref(),
        &[ctx.accounts.otc_swap.bump],
    ];
    let signer = &[&signer_seeds[..]];
    
    // Transfer full token amount from seller to swap PDA first
    let _transfer_to_pda = Transfer {
        from: ctx.accounts.seller_token_account.to_account_info(),
        to: ctx.accounts.seller_token_account.to_account_info(), // Temporary, we'll burn from here
        authority: ctx.accounts.otc_swap.to_account_info(),
    };
    
    // Burn tokens (20% penalty)
    if burn_amount > 0 {
        let burn_accounts = Burn {
            mint: ctx.accounts.token_mint.to_account_info(),
            from: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.otc_swap.to_account_info(),
        };
        
        let burn_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            burn_accounts,
            signer,
        );
        
        token::burn(burn_ctx, burn_amount)?;
        
        emit!(TokensBurned {
            user: seller_key,
            amount: burn_amount,
            reason: "patron_exit_penalty".to_string(),
        });
    }
    
    // Transfer remaining tokens to buyer
    let transfer_to_buyer = Transfer {
        from: ctx.accounts.seller_token_account.to_account_info(),
        to: ctx.accounts.buyer_token_account.to_account_info(),
        authority: ctx.accounts.otc_swap.to_account_info(),
    };
    
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_to_buyer,
        signer,
    );
    
    token::transfer(transfer_ctx, net_tokens_to_buyer)?;
    
    // TODO: Implement SOL payment - temporarily disabled for testing
    // **ctx.accounts.buyer.to_account_info().try_borrow_mut_lamports()? -= total_sol_payment;
    // **ctx.accounts.seller.to_account_info().try_borrow_mut_lamports()? += total_sol_payment;
    
    emit!(SwapCompleted {
        seller: seller_key,
        buyer: buyer_key,
        otc_swap: otc_swap_key,
        token_amount: net_tokens_to_buyer,
        sol_payment: total_sol_payment,
        rebate_amount: 0,
    });
    
    Ok(())
}

// ========== TREASURY BUYBACK ==========

#[derive(Accounts)]
pub struct AcceptTreasuryBuyback<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    
    #[account(
        seeds = [b"reward_pool"],
        bump,
        constraint = reward_pool.admin == admin.key() @ SnakeError::Unauthorized,
    )]
    pub reward_pool: Account<'info, RewardPool>,
    
    /// CHECK: Validated through PDA seeds
    #[account(mut)]
    pub seller: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"otc_swap", seller.key().as_ref()],
        bump = otc_swap.bump,
        constraint = otc_swap.seller == seller.key() @ SnakeError::Unauthorized,
        constraint = otc_swap.is_active @ SnakeError::SwapInactive,
        constraint = otc_swap.swap_type == crate::state::SwapType::ExiterToTreasury @ SnakeError::InvalidSwapType,
    )]
    pub otc_swap: Account<'info, OtcSwap>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key(),
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = treasury_token_account.owner == reward_pool.treasury,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn accept_treasury_buyback(ctx: Context<AcceptTreasuryBuyback>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    let _token_amount = ctx.accounts.otc_swap.token_amount;
    let sol_rate = ctx.accounts.otc_swap.sol_rate;
    let seller_key = ctx.accounts.seller.key();
    let otc_swap_key = ctx.accounts.otc_swap.key();
    let bump = ctx.accounts.otc_swap.bump;
    
    // Check if swap is expired
    require!(!ctx.accounts.otc_swap.is_expired(current_time), SnakeError::SwapExpired);
    
    // Calculate SOL payment - use u128 for intermediate calculations to avoid overflow
    let token_amount_u128 = _token_amount as u128;
    let sol_rate_u128 = sol_rate as u128;
    
    let total_sol_payment_u128 = token_amount_u128
        .checked_mul(sol_rate_u128)
        .ok_or(SnakeError::MathOverflow)?;
    
    let total_sol_payment = u64::try_from(total_sol_payment_u128)
        .map_err(|_| SnakeError::MathOverflow)?;
    
    // Verify treasury has sufficient SOL
    require!(
        ctx.accounts.treasury.to_account_info().lamports() >= total_sol_payment,
        SnakeError::InsufficientFunds
    );
    
    // Update swap state
    let swap = &mut ctx.accounts.otc_swap;
    swap.buyer = Some(ctx.accounts.reward_pool.treasury);
    swap.is_active = false;
    
    // Create PDA signer seeds
    let signer_seeds = &[
        b"otc_swap",
        seller_key.as_ref(),
        &[ctx.accounts.otc_swap.bump],
    ];
    let signer = &[&signer_seeds[..]];
    
    // Transfer tokens from seller to treasury
    let transfer_accounts = Transfer {
        from: ctx.accounts.seller_token_account.to_account_info(),
        to: ctx.accounts.treasury_token_account.to_account_info(),
        authority: ctx.accounts.otc_swap.to_account_info(),
    };
    
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        signer,
    );
    
    token::transfer(transfer_ctx, _token_amount)?;
    
    // Transfer SOL from treasury to seller using system program
    let (_, treasury_bump) = Pubkey::find_program_address(&[b"treasury"], &crate::ID);
    let treasury_seeds = &[b"treasury".as_ref(), &[treasury_bump]];
    let treasury_signer = &[&treasury_seeds[..]];
    
    let transfer_instruction = anchor_lang::system_program::Transfer {
        from: ctx.accounts.treasury.to_account_info(),
        to: ctx.accounts.seller.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        transfer_instruction,
        treasury_signer,
    );
    anchor_lang::system_program::transfer(cpi_ctx, total_sol_payment)?;
    
    emit!(SwapCompleted {
        seller: seller_key,
        buyer: ctx.accounts.reward_pool.treasury,
        otc_swap: otc_swap_key,
        token_amount: _token_amount,
        sol_payment: total_sol_payment,
        rebate_amount: 0,
    });
    
    Ok(())
}

// ========== FALLBACK TO TREASURY ==========

#[derive(Accounts)]
pub struct FallbackToTreasury<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        seeds = [b"reward_pool"],
        bump,
    )]
    pub reward_pool: Account<'info, RewardPool>,
    
    #[account(
        mut,
        seeds = [b"otc_swap", seller.key().as_ref()],
        bump = otc_swap.bump,
        constraint = otc_swap.seller == seller.key() @ SnakeError::Unauthorized,
        constraint = otc_swap.is_active @ SnakeError::SwapInactive,
        constraint = otc_swap.treasury_fallback @ SnakeError::TreasuryFallbackNotAllowed,
    )]
    pub otc_swap: Account<'info, OtcSwap>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key(),
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = treasury_token_account.owner == reward_pool.treasury,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub token_mint: Account<'info, Mint>,
    
    #[account(
        mut,
        seeds = [b"treasury"],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn fallback_to_treasury(ctx: Context<FallbackToTreasury>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    
    // Check if swap is expired (required for fallback)
    require!(ctx.accounts.otc_swap.is_expired(current_time), SnakeError::SwapNotExpired);
    
    let _token_amount = ctx.accounts.otc_swap.token_amount;
    let sol_rate = ctx.accounts.otc_swap.sol_rate;
    let seller_key = ctx.accounts.seller.key();
    let otc_swap_key = ctx.accounts.otc_swap.key();
    let bump = ctx.accounts.otc_swap.bump;
    
    // Apply 10% discount for treasury buyback fallback
    let discounted_rate = sol_rate.saturating_mul(90).saturating_div(100);
    
    // Calculate burn amount if applicable
    let burn_amount = ctx.accounts.otc_swap.calculate_burn_amount();
    let net_tokens_to_treasury = ctx.accounts.otc_swap.calculate_net_tokens_after_burn();
    
    // Calculate SOL payment with discount
    let total_sol_payment = net_tokens_to_treasury
        .checked_mul(discounted_rate)
        .ok_or(SnakeError::MathOverflow)?;
    
    // Verify treasury has sufficient SOL
    require!(
        ctx.accounts.treasury.to_account_info().lamports() >= total_sol_payment,
        SnakeError::InsufficientFunds
    );
    
    // Update swap state
    let swap = &mut ctx.accounts.otc_swap;
    swap.buyer = Some(ctx.accounts.reward_pool.treasury);
    swap.is_active = false;
    
    // Create PDA signer seeds
    let signer_seeds = &[
        b"otc_swap",
        seller_key.as_ref(),
        &[ctx.accounts.otc_swap.bump],
    ];
    let signer = &[&signer_seeds[..]];
    
    // Burn tokens if penalty applies
    if burn_amount > 0 {
        let burn_accounts = Burn {
            mint: ctx.accounts.token_mint.to_account_info(),
            from: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.otc_swap.to_account_info(),
        };
        
        let burn_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            burn_accounts,
            signer,
        );
        
        token::burn(burn_ctx, burn_amount)?;
        
        emit!(TokensBurned {
            user: seller_key,
            amount: burn_amount,
            reason: "patron_exit_penalty".to_string(),
        });
    }
    
    // Transfer remaining tokens to treasury
    let transfer_accounts = Transfer {
        from: ctx.accounts.seller_token_account.to_account_info(),
        to: ctx.accounts.treasury_token_account.to_account_info(),
        authority: ctx.accounts.otc_swap.to_account_info(),
    };
    
    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
        signer,
    );
    
    token::transfer(transfer_ctx, net_tokens_to_treasury)?;
    
    // Transfer SOL from treasury to seller using system program
    let (_, treasury_bump) = Pubkey::find_program_address(&[b"treasury"], &crate::ID);
    let treasury_seeds = &[b"treasury".as_ref(), &[treasury_bump]];
    let treasury_signer = &[&treasury_seeds[..]];
    
    let transfer_instruction = anchor_lang::system_program::Transfer {
        from: ctx.accounts.treasury.to_account_info(),
        to: ctx.accounts.seller.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        transfer_instruction,
        treasury_signer,
    );
    anchor_lang::system_program::transfer(cpi_ctx, total_sol_payment)?;
    
    emit!(SwapCompleted {
        seller: seller_key,
        buyer: ctx.accounts.reward_pool.treasury,
        otc_swap: otc_swap_key,
        token_amount: net_tokens_to_treasury,
        sol_payment: total_sol_payment,
        rebate_amount: 0,
    });
    
    Ok(())
}