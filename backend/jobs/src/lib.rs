mod twitter_api;

use std::error::Error;
use twitter_api::TwitterClient;

pub async fn run() -> Result<(), Box<dyn Error>> {
    // Initialize client
    let bearer_token = std::env::var("TWITTER_BEARER_TOKEN")?;
    let client = TwitterClient::new(bearer_token);

    // Example 1: Get recent tweets
    let response = client
        .get_tweets(
            "playSnakeAI",
            "MineTheSnake",
            &None, // No starting tweet ID
            Some(10),
        )
        .await?;

    println!("Recent tweets:");
    for tweet in response.data {
        println!("[{}] {}: {}", tweet.created_at, tweet.author_id, tweet.text);
    }

    // Example 2: Get all tweets starting from a specific ID
    let all_tweets = client
        .get_all_tweets(
            "playSnakeAI",
            "MineTheSnake",
            &Some("1234567890123456789".to_string()), // Starting tweet ID
            100,
        )
        .await?;

    println!("All tweets in chronological order:");
    for tweet in all_tweets {
        println!("[{}] {}: {}", tweet.created_at, tweet.author_id, tweet.text);
    }

    Ok(())
}
