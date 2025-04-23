use anyhow::anyhow;
use chrono::{DateTime, Days, Duration, Utc};
use database::AppService;
use qrcode_generator::QrCodeEcc;
use reqwest::multipart::{Form, Part};
use reqwest_oauth1::{OAuthClientProvider, Secrets};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashSet, sync::Arc};
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

    pub async fn send_message(&self, media_id: &str, tweet_id: &str) -> Result<(), anyhow::Error> {
        let endpoint = "https://api.twitter.com/2/tweets";

        let secrets = Secrets::new(&self.api_key, &self.api_key_secret)
            .token(&self.access_token, &self.access_token_secret);

        let payload = json!({
            "text": "Scan QR code to claim",
            "media": {
                "media_ids": [media_id]
            },
            "reply": {
                "in_reply_to_tweet_id": tweet_id
            }
        });

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
            let created_at = DateTime::parse_from_rfc3339(&t.created_at)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or(Utc::now());

            // Insert tweet
            if let Ok(tweet) = service
                .tweet
                .insert_tweet(&user.id, &t.id, &created_at)
                .await
            {
                // Create reward if conditions are met
                let should_create_reward =
                    match service.reward.get_available_reward(&user.id).await? {
                        Some(_) => false, // Reward already exists
                        None => match user.latest_claim_timestamp {
                            Some(timestamp) => timestamp
                                .checked_add_days(Days::new(1))
                                .map(|next_claim_date| next_claim_date < Utc::now())
                                .unwrap_or(false),
                            None => true, // No previous claim
                        },
                    };

                println!("log: should_create_reward: {}", should_create_reward);

                if should_create_reward && follow_users.contains(&user.twitter_id) {
                    service.reward.insert_reward(&user.id, &tweet.id).await.ok();
                    cnt += 1;
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
        "log: tweets: {:?}\n    follow users: {:?}",
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
            let media_id = if !reward.media_available() {
                match client
                    .upload_media(&format!("{}/claim/{}", env.frontend_url, reward.id))
                    .await
                {
                    Ok((media_id, expires_after_secs)) => {
                        match service
                            .reward
                            .update_reward_media_data(
                                &reward.id,
                                &media_id,
                                &(Utc::now() + Duration::seconds(expires_after_secs)),
                            )
                            .await
                        {
                            Ok(ok) => ok.media_id,
                            Err(err) => {
                                println!("update db error: {:?}", err);
                                continue;
                            }
                        }
                    }
                    Err(err) => {
                        println!("upload_media error: {:?}", err);
                        continue;
                    }
                }
            } else {
                reward.media_id.clone()
            };
            if media_id.is_none() {
                continue;
            }

            match client
                .send_message(&media_id.unwrap(), &reward.tweet_id)
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
