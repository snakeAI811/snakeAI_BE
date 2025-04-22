use chrono::{DateTime, Days, Utc};
use database::AppService;
use qrcode_generator::QrCodeEcc;
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};
use thiserror::Error;
use twitter_v2::TwitterApi;
use twitter_v2::authorization::Oauth1aToken;
use twitter_v2::id::NumericId;
use utils::env::Env;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Custom error: {0}")]
    Custom(String), // Variant that holds a String message
}
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

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaData {
    pub expires_after_secs: Option<i64>,
    pub id: Option<String>,
    pub media_key: Option<String>,
    pub size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadMediaResponse {
    pub data: Option<MediaData>,
}

pub struct TwitterClient {
    client: reqwest::Client,
    bearer_token: String,
    access_token: String,
    access_token_secret: String,
    api_key: String,
    api_key_secret: String,
}

impl TwitterClient {
    pub fn new(
        bearer_token: String,
        access_token: String,
        access_token_secret: String,
        api_key: String,
        api_key_secret: String,
    ) -> Self {
        Self {
            client: reqwest::Client::new(),
            bearer_token,
            access_token,
            access_token_secret,
            api_key,
            api_key_secret,
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
                .bearer_auth(&self.bearer_token)
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

    pub async fn upload_media(&self, text: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("https://api.twitter.com/2/media/upload");

        let qrcode: Vec<u8> = qrcode_generator::to_png_to_vec(text, QrCodeEcc::Low, 256).unwrap();

        let response = self
            .client
            .post(url)
            .bearer_auth(&self.bearer_token)
            .multipart(
                multipart::Form::new().part(
                    "file",
                    multipart::Part::bytes(qrcode)
                        .file_name("qrcode")
                        .mime_str("image/png")?,
                ),
            )
            .send()
            .await?
            .json::<UploadMediaResponse>()
            .await?;

        if let Some(data) = response.data {
            if let Some(id) = data.id {
                return Ok(id);
            }
        }

        return Err(Box::new(MyError::Custom("Upload media failed".to_string())));
    }

    pub async fn send_message(&self, text: &str, tweet_id: &str) -> Result<(), Box<dyn Error>> {
        let media_id = self.upload_media(text).await?;

        let vec: Vec<u64> = vec![];
        let tweet = TwitterApi::new(Oauth1aToken::new(
            &self.api_key,
            &self.api_key_secret,
            &self.access_token,
            &self.access_token_secret,
        ))
        .post_tweet()
        .add_media(vec![NumericId::new(media_id.parse().unwrap())], vec)
        .in_reply_to_tweet_id(NumericId::new(tweet_id.parse().unwrap()))
        .send()
        .await?;
        println!("{:?}", tweet);
        Ok(())
    }
}

pub async fn run(service: Arc<AppService>, env: Env) -> Result<(), Box<dyn Error>> {
    let client = TwitterClient::new(
        env.twitter_bearer_token,
        env.twitter_access_token,
        env.twitter_access_token_secret,
        env.twitter_api_key,
        env.twitter_api_key_secret,
    );

    // Fetch latest_tweet_id what I fetched last
    let latest_tweet_id = service.util.get_latest_tweet_id().await?;

    // Fetch new tweets with author information
    let (new_tweets, latest_tweet_id) = client
        .get_tweets("playSnakeAI", "MineTheSnake", latest_tweet_id)
        .await?;

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

    // Update latest_tweet_id from response
    if let Some(latest_tweet_id) = latest_tweet_id {
        service
            .util
            .upsert_latest_tweet_id(&latest_tweet_id)
            .await?;
    }

    if let Ok(rewards) = service.reward.get_rewards_to_send_message().await {
        for reward in &rewards {
            if client
                .send_message(
                    &format!("{}/claim/{}", env.frontend_url, reward.id),
                    &reward.tweet_id,
                )
                .await
                .is_ok()
            {
                service.reward.mark_as_message_sent(&reward.id).await.ok();
            }
        }
    }

    Ok(())
}
