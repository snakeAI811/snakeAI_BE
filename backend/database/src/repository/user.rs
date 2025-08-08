use crate::pool::DatabasePool;
use sqlx::types::{
    chrono::{DateTime, Utc},
    Uuid,
};
use std::sync::Arc;
use types::model::User;

#[derive(Clone)]
pub struct UserRepository {
    db_conn: Arc<DatabasePool>,
}

impl UserRepository {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    pub async fn insert_user(
        &self,
        twitter_id: &str,
        twitter_username: &str,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (twitter_id, twitter_username)
            VALUES ($1, $2)
            ON CONFLICT (twitter_id) DO UPDATE
            SET twitter_username = EXCLUDED.twitter_username
            RETURNING 
                id, twitter_id, twitter_username, wallet_address, latest_claim_timestamp, created_at, updated_at,
                role, selected_role, patron_status, locked_amount, lock_start_timestamp, lock_end_timestamp,
                lock_duration_months, last_yield_claim_timestamp, total_yield_claimed, user_claim_pda,
                initialized, vesting_pda, has_vesting, vesting_amount, vesting_role_type, otc_swap_count,
                total_burned, dao_eligibility_revoked_at, patron_qualification_score, wallet_age_days, community_score,
                role_transaction_signature, role_updated_at,  is_following
            "#,
            twitter_id,
            twitter_username,
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_user_id(&self, user_id: &Uuid) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
            .fetch_one(self.db_conn.get_pool())
            .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<Option<User>, sqlx::Error> {
        let parsed_id = match Uuid::parse_str(user_id) {
            Ok(id) => id,
            Err(_) => return Ok(None),
        };

        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", parsed_id)
            .fetch_optional(self.db_conn.get_pool())
            .await?;

        Ok(user)
    }

    pub async fn get_user_by_twitter_id(
        &self,
        twitter_id: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(User, "SELECT * FROM users WHERE twitter_id = $1", twitter_id)
            .fetch_optional(self.db_conn.get_pool())
            .await?;

        Ok(user)
    }

    pub async fn get_user_by_wallet_address(
        &self,
        wallet_address: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE wallet_address = $1",
            wallet_address
        )
        .fetch_optional(self.db_conn.get_pool())
        .await?;

        Ok(user)
    }

    pub async fn set_wallet_address_by_uuid(
        &self,
        user_id: &Uuid,
        wallet_address: &str,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users SET wallet_address = $1 WHERE id = $2 RETURNING *",
            wallet_address,
            user_id,
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(user)
    }

    pub async fn set_latest_claim_timestamp(
        &self,
        user_id: &Uuid,
        latest_claim_timestamp: &DateTime<Utc>,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users SET latest_claim_timestamp = $1 WHERE id = $2 RETURNING *",
            latest_claim_timestamp,
            user_id,
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(user)
    }

    // Patron Framework methods
    pub async fn update_patron_status(
        &self,
        user_id: &Uuid,
        patron_status: &str,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users SET patron_status = $1 WHERE id = $2 RETURNING *",
            patron_status,
            user_id,
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(user)
    }

    pub async fn update_role(
        &self,
        user_id: &Uuid,
        role: &str,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users SET role = $1 WHERE id = $2 RETURNING *",
            role,
            user_id,
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(user)
    }

    pub async fn update_lock_details(
        &self,
        user_id: &Uuid,
        lock_duration_months: i32,
        locked_amount: i64,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users SET lock_duration_months = $1, locked_amount = $2 WHERE id = $3 RETURNING *",
            lock_duration_months,
            locked_amount,
            user_id,
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(user)
    }

    pub async fn set_wallet_address(
        &self,
        user_id: &str,
        wallet_address: &str,
    ) -> Result<User, sqlx::Error> {
        let parsed_id = Uuid::parse_str(user_id)
            .map_err(|_| sqlx::Error::RowNotFound)?;

        let user = sqlx::query_as!(
            User,
            "UPDATE users SET wallet_address = $1 WHERE id = $2 RETURNING *",
            wallet_address,
            parsed_id,
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(user)
    }

    pub async fn update_is_following_by_twitter_id(
        &self,
        twitter_id: &str,
        is_following: bool,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users SET is_following = $1 WHERE twitter_id = $2 RETURNING *",
            is_following,
            twitter_id
        )
        .fetch_one(self.db_conn.get_pool())
        .await?;

        Ok(user)
    }

}
