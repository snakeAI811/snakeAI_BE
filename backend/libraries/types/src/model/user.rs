use anchor_client::solana_sdk::pubkey::Pubkey;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, sqlx::FromRow, Default, Debug)]
pub struct User {
    pub id: Uuid,
    pub twitter_id: String,
    pub twitter_username: Option<String>,
    pub wallet_address: Option<String>,
    pub latest_claim_timestamp: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    // Patron Framework fields  
    pub role: Option<String>, // Maps to 'role' column in DB
    pub selected_role: Option<String>,
    pub patron_status: Option<String>,
    pub locked_amount: Option<i64>,
    pub lock_start_timestamp: Option<DateTime<Utc>>,
    pub lock_end_timestamp: Option<DateTime<Utc>>,
    pub lock_duration_months: Option<i32>,
    pub last_yield_claim_timestamp: Option<DateTime<Utc>>,
    pub total_yield_claimed: Option<i64>,
    pub user_claim_pda: Option<String>,
    pub initialized: Option<bool>,
    pub vesting_pda: Option<String>,
    pub has_vesting: Option<bool>,
    pub vesting_amount: Option<i64>,
    pub vesting_role_type: Option<String>,
    pub otc_swap_count: Option<i32>,
    pub total_burned: Option<i64>,
    pub dao_eligibility_revoked_at: Option<DateTime<Utc>>,
    pub patron_qualification_score: Option<i32>,
    pub wallet_age_days: Option<i32>,
    pub community_score: Option<i32>,
    pub role_transaction_signature: Option<String>,
    pub role_updated_at: Option<DateTime<Utc>>,
    pub is_following: bool,
    pub accumulated_reward: Option<i64>
}

// Extended user model for patron framework features
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ExtendedUser {
    pub id: Uuid,
    pub twitter_id: String,
    pub twitter_username: Option<String>,
    pub wallet_address: Option<String>,
    pub latest_claim_timestamp: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    // Patron Framework fields
    pub role: Option<String>,
    pub patron_status: Option<String>,
    pub locked_amount: Option<i64>,
    pub lock_start_timestamp: Option<DateTime<Utc>>,
    pub lock_end_timestamp: Option<DateTime<Utc>>,
    pub lock_duration_months: Option<i32>,
    pub last_yield_claim_timestamp: Option<DateTime<Utc>>,
    pub total_yield_claimed: Option<i64>,
    pub user_claim_pda: Option<String>,
    pub initialized: Option<bool>,
    pub vesting_pda: Option<String>,
    pub has_vesting: Option<bool>,
    pub vesting_amount: Option<i64>,
    pub vesting_role_type: Option<String>,
    pub otc_swap_count: Option<i32>,
    pub total_burned: Option<i64>,
    pub dao_eligibility_revoked_at: Option<DateTime<Utc>>,
}

impl From<User> for ExtendedUser {
    fn from(user: User) -> Self {
        ExtendedUser {
            id: user.id,
            twitter_id: user.twitter_id,
            twitter_username: user.twitter_username,
            wallet_address: user.wallet_address,
            latest_claim_timestamp: user.latest_claim_timestamp,
            created_at: user.created_at,
            role: None,
            patron_status: None,
            locked_amount: None,
            lock_start_timestamp: None,
            lock_end_timestamp: None,
            lock_duration_months: None,
            last_yield_claim_timestamp: None,
            total_yield_claimed: None,
            user_claim_pda: None,
            initialized: None,
            vesting_pda: None,
            has_vesting: None,
            vesting_amount: None,
            vesting_role_type: None,
            otc_swap_count: None,
            total_burned: None,
            dao_eligibility_revoked_at: None,
        }
    }
}

impl User {
    pub fn wallet(&self) -> Option<Pubkey> {
        self.wallet_address
            .as_ref()
            .and_then(|addr| Pubkey::from_str(addr).ok())
    }
}

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
pub struct Profile {
    pub twitter_username: String,
    pub wallet_address: String,
    pub latest_claim_timestamp: Option<DateTime<Utc>>,
    pub reward_balance: i64,
    pub claimable_rewards: i64,
    pub tweets: i64,
    pub likes: i64,
    pub replies: i64,
    pub accumulated_reward: i64,
}
