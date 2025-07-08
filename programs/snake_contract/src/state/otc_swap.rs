use anchor_lang::prelude::*;
use crate::state::UserRole;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum SwapType {
    NormalToPatron,    // Normal users → Patron
    NormalToStaker,    // Normal users → Staker
    PatronToPatron,    // New: Patron → Patron (with 20% burn)
    TreasuryBuyback,   // New: Any → Treasury (fallback)
    AnyToAny,          // Open market
}

#[account]
#[derive(InitSpace)]
pub struct OtcSwap {
    pub seller: Pubkey,
    pub buyer: Option<Pubkey>,
    pub token_amount: u64,
    pub sol_rate: u64,           // SOL per token (in lamports)
    pub buyer_rebate: u64,       // Rebate percentage (basis points)
    pub seller_role: UserRole,
    pub buyer_role_required: UserRole,
    pub swap_type: SwapType,
    pub is_active: bool,
    pub created_at: i64,
    pub expires_at: i64,         // Expiration timestamp
    pub treasury_fallback: bool, // Allow treasury buyback if no buyer
    pub burn_penalty_rate: u64,  // Burn penalty for patron exits (basis points)
    pub bump: u8,
}

impl OtcSwap {
    pub fn init_normal_to_patron(
        &mut self,
        seller: Pubkey,
        token_amount: u64,
        sol_rate: u64,
        buyer_rebate: u64,
        seller_role: UserRole,
        current_time: i64,
        bump: u8,
    ) {
        self.seller = seller;
        self.buyer = None;
        self.token_amount = token_amount;
        self.sol_rate = sol_rate;
        self.buyer_rebate = buyer_rebate;
        self.seller_role = seller_role;
        self.buyer_role_required = UserRole::Patron;
        self.swap_type = SwapType::NormalToPatron;
        self.is_active = true;
        self.created_at = current_time;
        self.expires_at = current_time + (7 * 24 * 60 * 60); // 7 days
        self.treasury_fallback = false;
        self.burn_penalty_rate = 0;
        self.bump = bump;
    }
    
    pub fn init_patron_to_patron(
        &mut self,
        seller: Pubkey,
        token_amount: u64,
        sol_rate: u64,
        seller_role: UserRole,
        current_time: i64,
        bump: u8,
    ) {
        self.seller = seller;
        self.buyer = None;
        self.token_amount = token_amount;
        self.sol_rate = sol_rate;
        self.buyer_rebate = 0; // No rebate for patron-to-patron
        self.seller_role = seller_role;
        self.buyer_role_required = UserRole::Patron;
        self.swap_type = SwapType::PatronToPatron;
        self.is_active = true;
        self.created_at = current_time;
        self.expires_at = current_time + (7 * 24 * 60 * 60); // 7 days
        self.treasury_fallback = true; // Allow treasury fallback
        self.burn_penalty_rate = 2000; // 20% burn penalty
        self.bump = bump;
    }
    
    pub fn init_treasury_buyback(
        &mut self,
        seller: Pubkey,
        token_amount: u64,
        sol_rate: u64,
        seller_role: UserRole,
        current_time: i64,
        bump: u8,
    ) {
        self.seller = seller;
        self.buyer = None;
        self.token_amount = token_amount;
        self.sol_rate = sol_rate;
        self.buyer_rebate = 0;
        self.seller_role = seller_role;
        self.buyer_role_required = UserRole::None; // Treasury can buy from anyone
        self.swap_type = SwapType::TreasuryBuyback;
        self.is_active = true;
        self.created_at = current_time;
        self.expires_at = current_time + (30 * 24 * 60 * 60); // 30 days
        self.treasury_fallback = false;
        self.burn_penalty_rate = 0;
        self.bump = bump;
    }
    
    pub fn is_expired(&self, current_time: i64) -> bool {
        current_time > self.expires_at
    }
    
    pub fn calculate_burn_amount(&self) -> u64 {
        if self.burn_penalty_rate == 0 {
            return 0;
        }
        
        self.token_amount
            .saturating_mul(self.burn_penalty_rate)
            .saturating_div(10000) // basis points
    }
    
    pub fn calculate_net_tokens_after_burn(&self) -> u64 {
        self.token_amount.saturating_sub(self.calculate_burn_amount())
    }
}
