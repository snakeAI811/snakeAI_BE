use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::{UserClaim, UserRole};
use crate::events::OTCTradeExecuted;
use crate::errors::SnakeError;

#[derive(Accounts)]
pub struct CreateOTCOrder<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", seller.key().as_ref()],
        bump,
        constraint = seller_claim.user == seller.key()
    )]
    pub seller_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == seller.key()
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = seller,
        space = OTCOrder::INIT_SPACE,
        seeds = [b"otc_order", seller.key().as_ref()],
        bump
    )]
    pub otc_order: Account<'info, OTCOrder>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ExecuteOTCOrder<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"user_claim", buyer.key().as_ref()],
        bump,
        constraint = buyer_claim.user == buyer.key()
    )]
    pub buyer_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [b"otc_order", otc_order.seller.as_ref()],
        bump,
        constraint = otc_order.is_active
    )]
    pub otc_order: Account<'info, OTCOrder>,
    
    #[account(
        mut,
        constraint = seller_token_account.owner == otc_order.seller
    )]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = buyer_token_account.owner == buyer.key()
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = buyer_payment_account.owner == buyer.key()
    )]
    pub buyer_payment_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = seller_payment_account.owner == otc_order.seller
    )]
    pub seller_payment_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct OTCOrder {
    pub order_id: u64,
    pub seller: Pubkey,
    pub amount: u64,           // Amount of SNAKE tokens to sell
    pub price: u64,            // Price per token (in lamports or other token)
    pub is_active: bool,
    pub created_at: i64,
    pub buyer_restrictions: BuyerRestrictions, // Who can buy from this order
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct BuyerRestrictions {
    pub patrons_only: bool,
    pub treasury_only: bool,
    pub min_patron_score: u32,
}

impl Default for BuyerRestrictions {
    fn default() -> Self {
        Self {
            patrons_only: false,
            treasury_only: false,
            min_patron_score: 0,
        }
    }
}

pub fn create_otc_order(
    ctx: Context<CreateOTCOrder>,
    order_id: u64,
    amount: u64,
    price: u64,
    buyer_restrictions: BuyerRestrictions,
) -> Result<()> {
    let seller_claim = &ctx.accounts.seller_claim;
    let otc_order = &mut ctx.accounts.otc_order;
    
    // Only users who mined in Phase 1 can create sell orders
    require!(
        seller_claim.total_mined_phase1 > 0,
        SnakeError::NotEligibleForOTC
    );
    
    // Initialize OTC order
    otc_order.order_id = order_id;
    otc_order.seller = ctx.accounts.seller.key();
    otc_order.amount = amount;
    otc_order.price = price;
    otc_order.is_active = true;
    otc_order.created_at = Clock::get()?.unix_timestamp;
    otc_order.buyer_restrictions = buyer_restrictions;
    
    Ok(())
}

pub fn execute_otc_order(ctx: Context<ExecuteOTCOrder>) -> Result<()> {
    let buyer_claim = &ctx.accounts.buyer_claim;
    let otc_order = &mut ctx.accounts.otc_order;
    
    // Check buyer eligibility based on order restrictions
    if otc_order.buyer_restrictions.patrons_only {
        require!(
            buyer_claim.role == UserRole::Patron,
            SnakeError::PatronOnlyOrder
        );
    }
    
    if otc_order.buyer_restrictions.treasury_only {
        // Add treasury check logic here
        require!(
            false, // Placeholder - implement treasury check
            SnakeError::TreasuryOnlyOrder
        );
    }
    
    if otc_order.buyer_restrictions.min_patron_score > 0 {
        require!(
            buyer_claim.patron_qualification_score >= otc_order.buyer_restrictions.min_patron_score,
            SnakeError::InsufficientPatronScore
        );
    }
    
    // Transfer tokens from seller to buyer
    let transfer_tokens_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.seller_token_account.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: otc_order.to_account_info(),
        },
    );
    
    token::transfer(transfer_tokens_ctx, otc_order.amount)?;
    
    // Transfer payment from buyer to seller
    let transfer_payment_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.buyer_payment_account.to_account_info(),
            to: ctx.accounts.seller_payment_account.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        },
    );
    
    token::transfer(transfer_payment_ctx, otc_order.price)?;
    
    // Mark order as completed
    otc_order.is_active = false;
    
    // Emit event
    emit!(OTCTradeExecuted {
        order_id: otc_order.order_id,
        seller: otc_order.seller,
        buyer: ctx.accounts.buyer.key(),
        amount: otc_order.amount,
        price: otc_order.price,
    });
    
    Ok(())
}

pub fn cancel_otc_order(ctx: Context<CancelOTCOrder>) -> Result<()> {
    let otc_order = &mut ctx.accounts.otc_order;
    
    require!(
        otc_order.is_active,
        SnakeError::OrderNotActive
    );
    
    otc_order.is_active = false;
    
    Ok(())
}

#[derive(Accounts)]
pub struct CancelOTCOrder<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"otc_order", seller.key().as_ref()],
        bump,
        constraint = otc_order.seller == seller.key()
    )]
    pub otc_order: Account<'info, OTCOrder>,
}
