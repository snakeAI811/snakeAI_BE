use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OtcSwap {
    pub id: Uuid,
    pub seller_id: Uuid,
    pub buyer_id: Option<Uuid>,
    pub seller_wallet: String,
    pub buyer_wallet: Option<String>,
    pub otc_swap_pda: String,
    pub token_amount: i64,
    pub sol_rate: i64,
    pub buyer_rebate: i64,
    pub swap_type: String,
    pub buyer_role_required: String,
    pub status: String,
    pub initiate_tx_signature: Option<String>,
    pub accept_tx_signature: Option<String>,
    pub cancel_tx_signature: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOtcSwap {
    pub seller_id: Uuid,
    pub seller_wallet: String,
    pub otc_swap_pda: String,
    pub token_amount: i64,
    pub sol_rate: i64,
    pub buyer_rebate: i64,
    pub swap_type: String,
    pub buyer_role_required: String,
    pub initiate_tx_signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOtcSwap {
    pub buyer_id: Option<Uuid>,
    pub buyer_wallet: Option<String>,
    pub status: Option<String>,
    pub accept_tx_signature: Option<String>,
    pub cancel_tx_signature: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OtcSwapWithUsers {
    #[serde(flatten)]
    pub swap: OtcSwap,
    pub seller_username: Option<String>,
    pub buyer_username: Option<String>,
}

impl OtcSwap {
    pub fn is_active(&self) -> bool {
        self.status == "active" && self.expires_at > Utc::now()
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at <= Utc::now()
    }

    pub fn can_be_accepted_by(&self, user_role: &str) -> bool {
        if !self.is_active() {
            return false;
        }

        match self.buyer_role_required.as_str() {
            "none" => true,
            "staker" => user_role == "staker" || user_role == "patron",
            "patron" => user_role == "patron",
            _ => false,
        }
    }

    pub fn calculate_total_sol_payment(&self) -> Result<u64, String> {
        let token_amount = self.token_amount as u128;
        let sol_rate = self.sol_rate as u128;
        
        let total_payment = token_amount
            .checked_mul(sol_rate)
            .ok_or("Math overflow in SOL payment calculation")?;
        
        u64::try_from(total_payment)
            .map_err(|_| "SOL payment amount too large".to_string())
    }

    pub fn calculate_net_payment(&self) -> Result<u64, String> {
        let total_payment = self.calculate_total_sol_payment()?;
        let rebate = self.buyer_rebate as u64;
        
        total_payment
            .checked_sub(rebate)
            .ok_or("Rebate exceeds total payment".to_string())
    }
}