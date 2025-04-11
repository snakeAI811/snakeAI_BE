use chrono::{DateTime, Days, Utc};
use database::AppService;
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};
use utils::env::Env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub created_at: String,
    pub author_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetWithAuthor {
    pub id: String,
    pub created_at: String,
    pub author_id: String,
    pub author_username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterResponse {
    pub data: Option<Vec<Tweet>>,
    pub includes: Option<Includes>,
    pub meta: Option<TwitterMeta>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Includes {
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterMeta {
    pub newest_id: Option<String>,
    pub oldest_id: Option<String>,
    pub next_token: Option<String>,
    pub result_count: usize,
}

pub struct TwitterClient {
    client: reqwest::Client,
    bearer_token: String,
}

impl TwitterClient {
    pub fn new(bearer_token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            bearer_token,
        }
    }

    pub async fn get_tweets(
        &self,
        username: &str,
        hashtag: &str,
        latest_tweet_id: Option<String>,
    ) -> Result<(Vec<TweetWithAuthor>, Option<String>), Box<dyn Error>> {
        let query = format!("@{} #{}", username, hashtag);

        let mut url = format!(
            "https://api.twitter.com/2/tweets/search/recent?query={}&max_results=100&sort_order=recency&tweet.fields=created_at,author_id&expansions=author_id&user.fields=username",
            urlencoding::encode(&query)
        );
        if let Some(since_id) = &latest_tweet_id {
            url.push_str(&format!("&since_id={}", since_id));
        }

        let mut latest_tweet_id = latest_tweet_id;
        let mut result = vec![];

        loop {
            let response = self
                .client
                .get(&url)
                .bearer_auth(self.bearer_token.clone())
                .send()
                .await?
                .json::<TwitterResponse>()
                .await?;

            let users = response.includes.and_then(|i| i.users).unwrap_or_default();
            let tweets = response.data.unwrap_or_default();
            for tweet in &tweets {
                latest_tweet_id = match latest_tweet_id {
                    Some(current_id) if current_id < tweet.id => Some(tweet.id.clone()),
                    Some(current_id) => Some(current_id),
                    None => Some(tweet.id.clone()),
                }
            }

            result.extend(tweets.iter().map(|tweet| {
                let author = users.iter().find(|user| user.id == tweet.author_id);
                TweetWithAuthor {
                    id: tweet.id.clone(),
                    created_at: tweet.created_at.clone(),
                    author_id: tweet.author_id.clone(),
                    author_username: author.map(|author| author.username.clone()),
                }
            }));

            if let Some(meta) = response.meta {
                if let Some(next_token) = meta.next_token {
                    url = format!(
                        "https://api.twitter.com/2/tweets/search/recent?query={}&max_results=100&sort_order=recency&tweet.fields=created_at,author_id&expansions=author_id&user.fields=username&next_token={}",
                        urlencoding::encode(&query),
                        next_token,
                    );
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok((result, latest_tweet_id))
    }
}

pub async fn run(service: Arc<AppService>, env: Env) -> Result<(), Box<dyn Error>> {
    let client = TwitterClient::new(env.twitter_bearer_token);

    // Fetch latest_tweet_id what I fetched last
    let latest_tweet_id = service.util.get_latest_tweet_id().await?;

    // Fetch new tweets with author information
    let (new_tweets, latest_tweet_id) = client
        .get_tweets("playSnakeAI", "MineTheSnake", latest_tweet_id)
        .await?;

    // Update latest_tweet_id from response
    if let Some(latest_tweet_id) = latest_tweet_id {
        service
            .util
            .upsert_latest_tweet_id(&latest_tweet_id)
            .await?;
    }

    // Prepare tweets to insert into database
    for t in &new_tweets {
        // Get user
        let user = if let Some(user) = service.user.get_user_by_twitter_id(&t.author_id).await? {
            user
        } else {
            service
                .user
                .insert_user(&t.author_id, &t.author_username.clone().unwrap_or_default())
                .await?
        };

        let created_at = DateTime::parse_from_rfc3339(&t.created_at)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or(Utc::now());

        // Insert tweet
        let tweet = service
            .tweet
            .insert_tweet(&user.id, &t.id, &created_at)
            .await?;

        // Create reward if conditions are met
        let should_create_reward = match service.reward.get_available_reward(&user.id).await? {
            Some(_) => false, // Reward already exists
            None => match user.latest_claim_timestamp {
                Some(timestamp) => timestamp
                    .checked_add_days(Days::new(1))
                    .map(|next_claim_date| next_claim_date < Utc::now())
                    .unwrap_or(false),
                None => true, // No previous claim
            },
        };

        if should_create_reward {
            service.reward.insert_reward(&user.id, &tweet.id).await?;
        }
    }

    Ok(())
}
