#[derive(Debug, Clone, PartialEq)]
pub enum MiningPhase {
    Phase1,
    Phase2,
}

pub fn get_current_mining_phase(tweet_count: i64) -> MiningPhase {
    if tweet_count < 1_000_000 {
        MiningPhase::Phase1
    } else {
        MiningPhase::Phase2
    }
}

pub fn get_reward_burn_amount(tweet_count: i64) -> (u64, u64) {
    match tweet_count {
        1..=200_000 => (375, 375),
        200_001..=500_000 => (150, 150),
        500_001..=1_000_000 => (60, 60),
        1_000_001..=3_500_000 => (40, 40),
        _ => (0, 0), // No reward after 3,500,000
    }
}