use anchor_lang::prelude::*;
use crate::state::SwapType;

/// OTC Swap event handling utilities
pub struct OtcSwapEvents;

impl OtcSwapEvents {
    /// Emit swap initiated event with common parameters
    pub fn emit_swap_initiated(
        seller: Pubkey,
        swap_type: SwapType,
        token_amount: u64,
        sol_rate: u64,
        otc_swap: Pubkey,
        buyer_rebate: u64,
    ) {
        emit!(crate::events::SwapInitiated {
            seller,
            buyer_role_required: swap_type_to_user_role(swap_type),
            otc_swap,
            token_amount,
            sol_rate,
            buyer_rebate,
        });
    }

    /// Emit swap completed event with common parameters
    pub fn emit_swap_completed(
        seller: Pubkey,
        buyer: Pubkey,
        token_amount: u64,
        sol_payment: u64,
        otc_swap: Pubkey,
        rebate_amount: u64,
    ) {
        emit!(crate::events::SwapCompleted {
            seller,
            buyer,
            otc_swap,
            token_amount,
            sol_payment,
            rebate_amount,
        });
    }

    /// Emit swap cancelled event
    pub fn emit_swap_cancelled(seller: Pubkey, otc_swap: Pubkey) {
        emit!(crate::events::SwapCancelled {
            seller,
            otc_swap,
        });
    }

    /// Emit tokens burned event
    pub fn emit_tokens_burned(
        user: Pubkey,
        amount: u64,
        reason: String,
    ) {
        emit!(crate::events::TokensBurned {
            user,
            amount,
            reason,
        });
    }
}

/// Convert SwapType to UserRole for event emission
fn swap_type_to_user_role(swap_type: SwapType) -> crate::state::UserRole {
    match swap_type {
        SwapType::ExiterToPatron => crate::state::UserRole::Patron,
        SwapType::ExiterToTreasury => crate::state::UserRole::None,
        SwapType::PatronToPatron => crate::state::UserRole::Patron,
    }
}

impl ToString for SwapType {
    fn to_string(&self) -> String {
        match self {
            SwapType::ExiterToPatron => "Patron".to_string(),
            SwapType::ExiterToTreasury => "Treasury".to_string(),
            SwapType::PatronToPatron => "Patron".to_string(),
        }
    }
}
