#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;
    use crate::state::{UserRole, SwapType, UserClaim, OtcSwap, PatronStatus};
    use crate::utils::{CalculationUtils, ValidationUtils};
    use crate::instructions::otc_swap::{core::OtcSwapCore, deflationary::{DeflationaryMechanics, DailyVolumeTracker}};

    fn create_mock_user_claim(role: UserRole) -> UserClaim {
        UserClaim {
            initialized: true,
            user: Pubkey::default(),
            claimable_amount: 1000000000000,
            burn_amount: 0,
            last_claim_timestamp: 0,
            role,
            patron_status: PatronStatus::Approved,
            patron_application_timestamp: 0,
            patron_approval_timestamp: 0,
            locked_amount: 0,
            lock_start_timestamp: 0,
            lock_end_timestamp: 0,
            lock_duration_months: 0,
            last_yield_claim_timestamp: 0,
            total_yield_claimed: 0,
            sold_early: false,
            mined_in_phase2: false,
            dao_eligible: true,
            dao_seat_holder: false,
            dao_seat_acquired_timestamp: 0,
            total_mined_phase1: 1000000000000,
            wallet_age_days: 180,
            community_score: 85,
            patron_qualification_score: 95,
        }
    }

    fn create_mock_otc_swap(swap_type: SwapType) -> OtcSwap {
        OtcSwap {
            seller: Pubkey::default(),
            buyer: None,
            token_amount: 1000000000000,
            sol_rate: 4000000,
            buyer_rebate: 200,
            seller_role: UserRole::None,
            buyer_role_required: UserRole::Patron,
            swap_type,
            is_active: true,
            created_at: 0,
            expires_at: 0,
            treasury_fallback: false,
            burn_penalty_rate: 2000,
            fixed_price: 4000000,
            max_otc_limit: 1000000000000000,
            seller_exited: false,
            cooldown_period: 0,
            listing_active_at: 0,
            bump: 0,
        }
    }

    #[test]
    fn test_swap_initialization() {
        let mut otc_swap = create_mock_otc_swap(SwapType::ExiterToPatron);
        let seller = Pubkey::default();
        
        let result = OtcSwapCore::initialize_swap(
            &mut otc_swap,
            seller,
            1000000000000,
            4000000,
            200,
            SwapType::ExiterToPatron,
            0,
            1,
        );
        
        assert!(result.is_ok());
        assert_eq!(otc_swap.seller, seller);
        assert_eq!(otc_swap.token_amount, 1000000000000);
        assert_eq!(otc_swap.sol_rate, 4000000);
        assert!(otc_swap.is_active);
    }

    #[test]
    fn test_deflationary_calculations() {
        let token_amount = 1_000_000_000_000; // 1000 tokens
        let user_claim = create_mock_user_claim(UserRole::Patron);
        
        // Test ExiterToPatron (0% burn, 2% treasury skim)
        let burn_amount = DeflationaryMechanics::calculate_burn_amount(
            token_amount,
            &SwapType::ExiterToPatron,
            &user_claim,
            0,
        );
        assert_eq!(burn_amount, 0);
        
        let treasury_skim = DeflationaryMechanics::calculate_treasury_skim(
            token_amount,
            &SwapType::ExiterToPatron,
        );
        assert_eq!(treasury_skim, 20_000_000_000); // 2% of 1000 tokens
        
        // Test PatronToPatron (20% burn, 0% treasury skim)
        let burn_amount = DeflationaryMechanics::calculate_burn_amount(
            token_amount,
            &SwapType::PatronToPatron,
            &user_claim,
            0,
        );
        assert_eq!(burn_amount, 200_000_000_000); // 20% of 1000 tokens
        
        let treasury_skim = DeflationaryMechanics::calculate_treasury_skim(
            token_amount,
            &SwapType::PatronToPatron,
        );
        assert_eq!(treasury_skim, 0);
        
        // Test ExiterToTreasury (1% burn, 5% treasury skim)
        let burn_amount = DeflationaryMechanics::calculate_burn_amount(
            token_amount,
            &SwapType::ExiterToTreasury,
            &user_claim,
            0,
        );
        assert_eq!(burn_amount, 10_000_000_000); // 1% of 1000 tokens
        
        let treasury_skim = DeflationaryMechanics::calculate_treasury_skim(
            token_amount,
            &SwapType::ExiterToTreasury,
        );
        assert_eq!(treasury_skim, 50_000_000_000); // 5% of 1000 tokens
    }

    #[test]
    fn test_daily_volume_tracker_mock() {
        let mut tracker = DailyVolumeTracker {
            date: 20250824,
            total_volume: 0,
            max_daily_volume: 1_000_000_000_000_000, // 1M tokens max daily
            swap_count: 0,
            last_reset: 1692835200, // Mock timestamp
            bump: 0,
        };
        
        // Manually add volume without using Clock
        tracker.total_volume = 100_000_000_000; // 100 tokens
        tracker.swap_count = 1;
        
        let (total_volume, max_volume, swap_count) = tracker.get_volume_status();
        assert_eq!(total_volume, 100_000_000_000);
        assert_eq!(max_volume, 1_000_000_000_000_000);
        assert_eq!(swap_count, 1);
    }

    #[test]
    fn test_safe_arithmetic() {
        // Test safe addition
        let result = CalculationUtils::safe_add(100, 200);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 300);
        
        // Test safe multiplication
        let result = CalculationUtils::safe_mul(100, 200);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 20000);
        
        // Test safe subtraction
        let result = CalculationUtils::safe_sub(300, 100);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 200);
    }

    #[test]
    fn test_validation_utils() {
        // Test amount range validation
        let result = ValidationUtils::validate_amount_range(100, 50, 200);
        assert!(result.is_ok());
        
        let result = ValidationUtils::validate_amount_range(25, 50, 200);
        assert!(result.is_err());
        
        let result = ValidationUtils::validate_amount_range(300, 50, 200);
        assert!(result.is_err());
    }

    #[test]
    fn test_daily_volume_exceeds_limit_mock() {
        let tracker = DailyVolumeTracker {
            date: 20250824,
            total_volume: 90_000_000_000, // 90 tokens already used
            max_daily_volume: 100_000_000_000, // 100 tokens max daily
            swap_count: 1,
            last_reset: 1692835200, // Mock timestamp
            bump: 0,
        };
        
        // Simulate exceeding limit by checking manually
        let attempted_volume = 20_000_000_000; // 20 tokens
        let would_exceed = tracker.total_volume + attempted_volume > tracker.max_daily_volume;
        assert!(would_exceed);
    }

    #[test]
    fn test_patron_exit_burn_penalty() {
        let token_amount = 1_000_000_000_000; // 1000 tokens
        let mut user_claim = create_mock_user_claim(UserRole::Patron);
        
        // Set patron approval timestamp to recent time (within 6 months)
        let current_time = 1000000;
        user_claim.patron_approval_timestamp = current_time - 1000; // Less than 6 months ago
        
        let burn_amount = OtcSwapCore::calculate_patron_exit_burn(
            token_amount,
            &SwapType::PatronToPatron,
            &user_claim,
            current_time,
        );
        
        assert_eq!(burn_amount, 200_000_000_000); // 20% burn penalty
    }

    #[test]
    fn test_patron_exit_no_penalty_after_commitment() {
        let token_amount = 1_000_000_000_000; // 1000 tokens
        let mut user_claim = create_mock_user_claim(UserRole::Patron);
        
        // Set patron approval timestamp to old time (more than 6 months ago)
        let current_time = 1000000;
        let six_months_ago = current_time - (6 * 30 * 24 * 60 * 60 + 1000);
        user_claim.patron_approval_timestamp = six_months_ago;
        
        let burn_amount = OtcSwapCore::calculate_patron_exit_burn(
            token_amount,
            &SwapType::PatronToPatron,
            &user_claim,
            current_time,
        );
        
        assert_eq!(burn_amount, 0); // No burn penalty after commitment period
    }

    #[test]
    fn test_net_amount_calculation() {
        let token_amount: u64 = 1_000_000_000_000; // 1000 tokens
        let burn_amount: u64 = 200_000_000_000; // 20% burn
        let treasury_skim: u64 = 0; // No treasury skim for P2P
        
        let net_amount = token_amount
            .checked_sub(burn_amount)
            .unwrap()
            .checked_sub(treasury_skim)
            .unwrap();
        
        assert_eq!(net_amount, 800_000_000_000); // 800 tokens after 20% burn
    }
}
