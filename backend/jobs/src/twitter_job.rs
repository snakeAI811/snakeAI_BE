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
        let query = format!("@{} AND #{} -is:reply", username, hashtag);

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

    let latest_tweet_id = service.util.get_latest_tweet_id().await?;

    let (new_tweets, latest_tweet_id) = client
        .get_tweets("playSnakeAI", "MineTheSnake", latest_tweet_id)
        .await?;

    // ✳️ Extract all tweet authors
    let author_ids: Vec<String> = new_tweets.iter().map(|t| t.author_id.clone()).collect();

    // ✳️ Check which authors are followers
    let current_followers = client.get_follow_users(author_ids.clone()).await?;

    // ✳️ Persist follower status to DB
    for twitter_id in author_ids {
        let is_following = current_followers.contains(&twitter_id);
        service
            .user
            .update_is_following_by_twitter_id(&twitter_id, is_following)
            .await
            .ok();
    }

    let mut cnt = 0;
    let mut tweet_count = service.tweet.get_tweets_count(None).await.unwrap_or(0);

    for t in &new_tweets {
        tweet_count += 1;

        let user = match service.user.get_user_by_twitter_id(&t.author_id).await? {
            Some(user) => Some(user),
            None => {
                let new_user = service
                    .user
                    .insert_user(&t.author_id, &t.author_username.clone().unwrap_or_default())
                    .await
                    .ok();

                // Set follow status if inserted
                if let Some(u) = &new_user {
                    let is_following = current_followers.contains(&u.twitter_id);
                    service
                        .user
                        .update_is_following_by_twitter_id(&u.twitter_id, is_following)
                        .await
                        .ok();
                }

                new_user
            }
        };

        if let Some(user) = user {
            if user.twitter_id == env.play_snake_ai_id {
                continue;
            }

            let created_at = DateTime::parse_from_rfc3339(&t.created_at)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or(Utc::now());

            let mining_phase = get_current_mining_phase(tweet_count);
            let (reward_amount, burn_amount) = get_reward_burn_amount(tweet_count);

            println!(
                "log: tweet_count = {}, mining_phase = {:?}, reward_amount = {}, burn_amount = {}",
                tweet_count, mining_phase, reward_amount, burn_amount
            );

            if let Ok(tweet) = service
                .tweet
                .insert_tweet(
                    &user.id,
                    &t.id,
                    &created_at,
                    &mining_phase.to_string(),
                    reward_amount,
                )
                .await
            {
                let mut text = String::new();

                let should_create_reward = if user.is_following {
                    match service.reward.get_available_reward(&user.id).await? {
                        Some(reward) => {
                            text = format!(
                                "🎁 You already have an unclaimed reward waiting! Don't miss out:\n\n🔗 Claim here: {}\n\n#SnakeAI",
                                reward.get_reward_url(&env.frontend_url)
                            );
                            false
                        }
                        None => match user.latest_claim_timestamp {
                            Some(timestamp) => {
                                let result = timestamp
                                    .checked_add_signed(Duration::minutes(25)) // ✅ cooldown check changed
                                    .map(|next_claim_date| next_claim_date < Utc::now())
                                    .unwrap_or(false);

                                if !result {
                                    let formatted_time =
                                        timestamp.format("%Y-%-m-%-d %H:%M:%S").to_string();
                                    let formatted_next_time = (timestamp + Duration::minutes(25))
                                        .format("%Y-%-m-%-d %H:%M:%S")
                                        .to_string();
                                    text = format!(
                                                "⏰ You claimed at {}. Next mining available after {} (25 min cooldown). Thanks for playing! 🐍",
                                                formatted_time, formatted_next_time
                                            );
                                }

                                result
                            }
                            None => true,
                        },
                    }
                } else {
                    text = "👋 To qualify for Snake AI token rewards, please follow @playSnakeAI first, then tweet again with @playSnakeAI & #MineTheSnake! 🐍".to_string();
                    false
                };

                println!(
                    "log: should_create_reward: {}, text: {}",
                    should_create_reward, text
                );

                if should_create_reward {
                    println!(
                        "log: creating reward for user_id = {}, tweet_id = {}, phase = {}, reward = {}, burn = {}",
                        user.id,
                        tweet.id,
                        if mining_phase == MiningPhase::Phase2 { 2 } else { 1 },
                        reward_amount,
                        burn_amount
                    );

                    service
                        .reward
                        .insert_reward_with_phase_and_amounts(
                            &user.id,
                            &tweet.id,
                            if mining_phase == MiningPhase::Phase2 {
                                2
                            } else {
                                1
                            },
                            reward_amount,
                            burn_amount,
                        )
                        .await
                        .ok();
                    cnt += 1;
                } else {
                    match client.reply_with_media(&text, &None, &t.id).await {
                        Ok(_) => {}
                        Err(err) => println!("send message error: {:?}", err),
                    }
                }
            }
        }
    }

    println!(
        "log: {} tweets detected, {} rewards created",
        new_tweets.len(),
        cnt
    );

    if let Some(latest_tweet_id) = latest_tweet_id {
        service
            .util
            .upsert_latest_tweet_id(&latest_tweet_id)
            .await?;
    }

    if let Ok(rewards) = service.reward.get_rewards_to_send_message().await {
        for reward in &rewards {
            if client
                .reply_with_media(
                    &format!(
                        "🎉 Congrats! Your tweet qualified for Snake AI token rewards! 🐍💰\n\n🔗 Claim your tokens: {}\n\n#SnakeAI #TokenRewards",
                        reward.get_reward_url(&env.frontend_url)
                    ),
                    &None,
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

fn get_current_mining_phase(tweet_count: i64) -> MiningPhase {
    if tweet_count < 1_000_000 {
        MiningPhase::Phase1
    } else {
        MiningPhase::Phase2
    }
}

fn get_reward_burn_amount(tweet_count: i64) -> (u64, u64) {
    match tweet_count {
        1..=200_000 => (375, 375),
        200_001..=500_000 => (150, 150),
        500_001..=1_000_000 => (60, 60),
        1_000_001..=3_500_000 => (40, 40),
        _ => (0, 0), // No reward after 3,500,000
    }
}
