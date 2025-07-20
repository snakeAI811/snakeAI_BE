use anyhow::anyhow;
use chrono::{DateTime, Days, Duration, Utc};
use database::AppService;
use qrcode_generator::QrCodeEcc;
use reqwest::multipart::{Form, Part};
use reqwest_oauth1::{OAuthClientProvider, Secrets};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashSet, sync::Arc};
use types::model::RewardUtils;
use utils::env::Env;

#[derive(Debug, Clone, PartialEq)]
pub enum MiningPhase {
    Phase1,
    Phase2,
}

impl MiningPhase {
    pub fn to_string(&self) -> String {
        match self {
            MiningPhase::Phase1 => "phase1".to_string(),
            MiningPhase::Phase2 => "phase2".to_string(),
        }
    }
    
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "phase1" => Some(MiningPhase::Phase1),
            "phase2" => Some(MiningPhase::Phase2),
            _ => None,
        }
    }
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
pub struct ProcessingInfo {
    pub check_after_secs: Option<i64>,
    pub progress_percent: Option<i64>,
    pub state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadMediaResponse {
    pub expires_after_secs: Option<i64>,
    pub id: Option<String>,
    pub size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    #[serde(default)]
    pub id: String,
    // #[serde(default)]
    // pub username: String,
    // #[serde(default)]
    // pub name: String,
    #[serde(default)]
    pub connection_status: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersResponse {
    #[serde(default)]
    pub data: Vec<UserData>,
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
    ) -> Result<(Vec<TweetWithAuthor>, Option<String>), anyhow::Error> {
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

    pub async fn upload_media(&self, text: &str) -> Result<(String, i64), anyhow::Error> {
        let endpoint = "https://api.twitter.com/2/media/upload";

        let file_content: Vec<u8> =
            qrcode_generator::to_png_to_vec(text, QrCodeEcc::Low, 256).unwrap();

        let secrets = Secrets::new(&self.api_key, &self.api_key_secret)
            .token(&self.access_token, &self.access_token_secret);

        // Create multipart form data
        let part = Part::bytes(file_content)
            .file_name("qrcode.png")
            .mime_str("image/png")?;
        let form = Form::new().part("media", part);

        let response = reqwest::Client::new()
            .oauth1(secrets)
            .post(endpoint)
            .multipart(form)
            .send()
            .await?
            .json::<UploadMediaResponse>()
            .await?;

        if let Some(id) = &response.id {
            return Ok((
                id.to_string(),
                response.expires_after_secs.unwrap_or_default(),
            ));
        }

        return Err(anyhow!(format!("Upload media failed: {:?}", response)));
    }

    pub async fn reply_with_media(
        &self,
        text: &str,
        media_id: &Option<String>,
        tweet_id: &str,
    ) -> Result<(), anyhow::Error> {
        let endpoint = "https://api.twitter.com/2/tweets";

        let secrets = Secrets::new(&self.api_key, &self.api_key_secret)
            .token(&self.access_token, &self.access_token_secret);

        let payload = if let Some(media_id) = media_id {
            json!({
                "text": text,
                "media": {
                    "media_ids": [media_id]
                },
                "reply": {
                    "in_reply_to_tweet_id": tweet_id
                }
            })
        } else {
            json!({
                "text": text,
                "reply": {
                    "in_reply_to_tweet_id": tweet_id
                }
            })
        };

        let response = reqwest::Client::new()
            .oauth1(secrets)
            .post(endpoint)
            .json(&payload)
            .send()
            .await?;

        println!("sent comment: {:?}", response);
        Ok(())
    }

    pub async fn get_follow_users(
        &self,
        twitter_ids: Vec<String>,
    ) -> Result<HashSet<String>, anyhow::Error> {
        let endpoint = "https://api.twitter.com/2/users";

        let secrets = Secrets::new(&self.api_key, &self.api_key_secret)
            .token(&self.access_token, &self.access_token_secret);

        let mut result = HashSet::new();

        for chunk in twitter_ids.chunks(100) {
            let ids = chunk.join(",");
            let response = reqwest::Client::new()
                .oauth1(secrets.clone())
                .get(endpoint)
                .query(&[("user.fields", "connection_status"), ("ids", &ids)])
                .send()
                .await?
                .json::<UsersResponse>()
                .await?;

            for user in response.data {
                if user.connection_status.contains(&"followed_by".to_string()) {
                    result.insert(user.id);
                }
            }
        }

        return Ok(result);
    }
}

pub async fn run(service: Arc<AppService>, env: Env) -> Result<(), anyhow::Error> {
    let client = TwitterClient::new(
        env.twitter_bearer_token.clone(),
        env.twitter_access_token.clone(),
        env.twitter_access_token_secret.clone(),
        env.twitter_api_key.clone(),
        env.twitter_api_key_secret.clone(),
    );

    // Fetch latest_tweet_id what I fetched last
    let latest_tweet_id = service.util.get_latest_tweet_id().await?;

    // Fetch new tweets with author information
    let (new_tweets, latest_tweet_id) = client
        .get_tweets("playSnakeAI", "MineTheSnake", latest_tweet_id)
        .await?;

    let follow_users = client
        .get_follow_users(
            new_tweets
                .iter()
                .map(|tweet| tweet.author_id.clone())
                .collect(),
        )
        .await?;

    let mut cnt = 0;

    // Prepare tweets to insert into database
    for t in &new_tweets {
        // Get user
        let user = if let Some(user) = service.user.get_user_by_twitter_id(&t.author_id).await? {
            Some(user)
        } else {
            service
                .user
                .insert_user(&t.author_id, &t.author_username.clone().unwrap_or_default())
                .await
                .ok()
        };

        if let Some(user) = user {
            if user.twitter_id == env.play_snake_ai_id {
                continue;
            }

            let created_at = DateTime::parse_from_rfc3339(&t.created_at)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or(Utc::now());

            // Determine mining phase based on tweet creation time
            let mining_phase = if env.get_mining_phase() == 2 { "Phase2" } else { "Phase1" };
            
            // Insert tweet
            if let Ok(tweet) = service
                .tweet
                .insert_tweet(&user.id, &t.id, &created_at, mining_phase)
                .await
            {
                // Create reward if conditions are met
                let mut text = String::new();

                let should_create_reward = match follow_users.contains(&user.twitter_id) {
                    true => {
                        // User follow us
                        match service.reward.get_available_reward(&user.id).await? {
                            Some(reward) => {
                                // Reward already exists
                                text = format!(
                                    "You have already available reward. Click the link to claim. {}",
                                    reward.get_reward_url(&env.frontend_url)
                                );

                                false
                            }
                            None => {
                                // There is no available reward
                                match user.latest_claim_timestamp {
                                    Some(timestamp) => {
                                        let result = timestamp
                                            .checked_add_days(Days::new(1))
                                            .map(|next_claim_date| next_claim_date < Utc::now())
                                            .unwrap_or(false);

                                        if !result {
                                            let formatted_time =
                                                timestamp.format("%Y-%-m-%-d %H:%M:%S").to_string();
                                            let next_time = timestamp + Duration::hours(24);
                                            let formatted_next_time =
                                                next_time.format("%Y-%-m-%-d %H:%M:%S").to_string();
                                            text = format!(
                                                "You've claimed reward at {}, post after {} (24 hours later) to mint",
                                                formatted_time, formatted_next_time
                                            );
                                        }

                                        result
                                    }
                                    None => true, // No previous claim
                                }
                            }
                        }
                    }
                    false => {
                        // User don't follow us
                        text = format!("You need to follow us before posting to get your rewards");
                        false
                    }
                };

                println!(
                    "log: should_create_reward: {}, text: {}",
                    should_create_reward, text
                );

                if should_create_reward {
                    // Determine current mining phase - this should be configurable
                    let current_phase = get_current_mining_phase().await;
                    service.reward.insert_reward_with_phase(&user.id, &tweet.id, if current_phase == MiningPhase::Phase2 { 2 } else { 1 }).await.ok();
                    cnt += 1;
                } else {
                    match client.reply_with_media(&text, &None, &t.id).await {
                        Ok(_) => {}
                        Err(err) => {
                            println!("send message error: {:?}", err);
                        }
                    }
                }
            }
        }
    }

    println!(
        "log: {} tweets detected, {} follow users, {} rewards created",
        new_tweets.len(),
        follow_users.len(),
        cnt
    );

    println!(
        "log: tweets: {:?}\n     follow users: {:?}",
        new_tweets, follow_users,
    );

    // Update latest_tweet_id from response
    if let Some(latest_tweet_id) = latest_tweet_id {
        service
            .util
            .upsert_latest_tweet_id(&latest_tweet_id)
            .await?;
    }

    if let Ok(rewards) = service.reward.get_rewards_to_send_message().await {
        for reward in &rewards {
            match client
                .reply_with_media(
                    &format!(
                        "Click the link to claim. {}",
                        reward.get_reward_url(&env.frontend_url)
                    ),
                    &None,
                    &reward.tweet_id,
                )
                .await
            {
                Ok(_) => {
                    service.reward.mark_as_message_sent(&reward.id).await.ok();
                }
                Err(err) => {
                    println!("send message error: {:?}", err);
                }
            }
        }
    }

    Ok(())
}

// Helper function to determine current mining phase
// This should be configurable based on project timeline
async fn get_current_mining_phase() -> MiningPhase {
    // For now, we'll use a simple time-based approach
    // In production, this should be configurable via database or config
    let current_time = Utc::now();
    
    // Example: Phase 1 ends on a specific date, then Phase 2 begins
    // This should be configurable
    let phase1_end = DateTime::parse_from_rfc3339("2024-06-01T00:00:00Z").unwrap().with_timezone(&Utc);
    
    if current_time < phase1_end {
        MiningPhase::Phase1
    } else {
        MiningPhase::Phase2
    }
}
