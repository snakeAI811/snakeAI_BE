use anchor_lang::prelude::*;
use crate::state::UserRole;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum SwapType {
    ExiterToPatron,    // Phase 1: Exiter (None role) → Patron at fixed price
    ExiterToTreasury,  // Phase 1: Exiter → Treasury (fallback)
    PatronToPatron,    // Phase 2: Patron → Patron with 20% burn
}

#[account]
#[derive(InitSpace)]
pub struct OtcSwap {
    pub seller: Pubkey,
    pub buyer: Option<Pubkey>,
    pub token_amount: u64,
    pub sol_rate: u64,           // Fixed price per token (in lamports)
    pub buyer_rebate: u64,       // Rebate percentage for Patrons (basis points)
    pub seller_role: UserRole,
    pub buyer_role_required: UserRole,
    pub swap_type: SwapType,
    pub is_active: bool,
    pub created_at: i64,
    pub expires_at: i64,         // Expiration timestamp
    pub treasury_fallback: bool, // Allow treasury buyback if no Patron available
    pub burn_penalty_rate: u64,  // Burn penalty for patron exits (basis points, 2000 = 20%)
    pub fixed_price: u64,        // Phase 1 fixed price (e.g., $0.0040 in lamports)
    pub max_otc_limit: u64,      // Maximum OTC limit per user
    pub seller_exited: bool,     // Mark seller as "exited" after swap
    pub cooldown_period: i64,    // Cooldown before listing becomes active
    pub listing_active_at: i64,  // When the listing becomes active
    pub bump: u8,
}

impl OtcSwap {
    // Phase 1: Exiter (None role) → Patron at fixed price
    pub fn init_exiter_to_patron(
        &mut self,
        seller: Pubkey,
        token_amount: u64,
        fixed_price: u64,        // Fixed price (e.g., $0.0040 per token)
        buyer_rebate: u64,       // Rebate for Patrons (e.g., 2%)
        max_otc_limit: u64,      // Max OTC limit per user
        current_time: i64,
        bump: u8,
    ) {
        self.seller = seller;
        self.buyer = None;
        self.token_amount = token_amount;
        self.sol_rate = fixed_price;
        self.fixed_price = fixed_price;
        self.buyer_rebate = buyer_rebate;
        self.seller_role = UserRole::None; // Must be None (Exiter)
        self.buyer_role_required = UserRole::Patron;
        self.swap_type = SwapType::ExiterToPatron;
        self.is_active = true;
        self.created_at = current_time;
        self.expires_at = current_time + (7 * 24 * 60 * 60); // 7 days
        self.treasury_fallback = true; // Treasury as fallback
        self.burn_penalty_rate = 0; // No burn for Phase 1
        self.max_otc_limit = max_otc_limit;
        self.seller_exited = false;
        self.cooldown_period = 0; // No cooldown for Phase 1
        self.listing_active_at = current_time; // Active immediately
        self.bump = bump;
    }
    
    // Phase 2: Patron → Patron with 20% burn on exit
    pub fn init_patron_to_patron(
        &mut self,
        seller: Pubkey,
        token_amount: u64,
        asking_price: u64,       // Patron's asking price
        cooldown_period: i64,    // Cooldown before listing becomes active
        current_time: i64,
        bump: u8,
    ) {
        self.seller = seller;
        self.buyer = None;
        self.token_amount = token_amount;
        self.sol_rate = asking_price;
        self.fixed_price = 0; // Not applicable for Phase 2
        self.buyer_rebate = 0; // No rebate for patron-to-patron
        self.seller_role = UserRole::Patron; // Must be Patron
        self.buyer_role_required = UserRole::Patron;
        self.swap_type = SwapType::PatronToPatron;
        self.is_active = false; // Will become active after cooldown
        self.created_at = current_time;
        self.expires_at = current_time + (30 * 24 * 60 * 60); // 30 days
        self.treasury_fallback = false; // No treasury fallback for P2P
        self.burn_penalty_rate = 2000; // 20% burn penalty
        self.max_otc_limit = 0; // No limit for patron exits
        self.seller_exited = false; // Will be marked true on completion
        self.cooldown_period = cooldown_period;
        self.listing_active_at = current_time + cooldown_period;
        self.bump = bump;
    }
    
    // Phase 1: Treasury buyback (fallback when no Patron available)
    pub fn init_treasury_buyback(
        &mut self,
        seller: Pubkey,
        token_amount: u64,
        fixed_price: u64,        // Same fixed price as ExiterToPatron
        current_time: i64,
        bump: u8,
    ) {
        self.seller = seller;
        self.buyer = None;
        self.token_amount = token_amount;
        self.sol_rate = fixed_price;
        self.fixed_price = fixed_price;
        self.buyer_rebate = 0; // No rebate for treasury
        self.seller_role = UserRole::None; // Exiter selling to treasury
        self.buyer_role_required = UserRole::None; // Treasury has no role requirement
        self.swap_type = SwapType::ExiterToTreasury;
        self.is_active = true;
        self.created_at = current_time;
        self.expires_at = current_time + (30 * 24 * 60 * 60); // 30 days
        self.treasury_fallback = false; // This IS the treasury buyback
        self.burn_penalty_rate = 0; // No burn for Phase 1
        self.max_otc_limit = 0; // No limit for treasury
        self.seller_exited = true; // Mark as exited when selling to treasury
        self.cooldown_period = 0;
        self.listing_active_at = current_time;
        self.bump = bump;
    }
    
    pub fn is_expired(&self, current_time: i64) -> bool {
        current_time > self.expires_at
    }
    
    pub fn is_listing_active(&self, current_time: i64) -> bool {
        current_time >= self.listing_active_at && self.is_active
    }
    
    pub fn calculate_burn_amount(&self) -> u64 {
        if self.burn_penalty_rate == 0 {
            return 0;
        }
        
        self.token_amount
            .saturating_mul(self.burn_penalty_rate)
            .saturating_div(10000) // basis points (2000 = 20%)
    }
    
    pub fn calculate_net_tokens_after_burn(&self) -> u64 {
        self.token_amount.saturating_sub(self.calculate_burn_amount())
    }
    
    pub fn mark_seller_as_exited(&mut self) {
        self.seller_exited = true;
        self.is_active = false;
    }
    
    pub fn activate_listing(&mut self, current_time: i64) {
        if current_time >= self.listing_active_at {
            self.is_active = true;
        }
    }
}
