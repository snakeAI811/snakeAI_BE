use crate::{error::DatabaseError, pool::DatabasePool, repository::TweetTemplateRepository};
use reqwest::Client;
use serde_json::json;
use sqlx::PgPool;
use std::{env, sync::Arc};
use types::model::{PostTweetRequest, PostTweetResponse, TweetTemplateResponse};
use serde_json::Value;
use sqlx::types::uuid;

#[derive(Clone)]
pub struct TweetTemplateService {
    db_pool: Arc<DatabasePool>,
}

impl TweetTemplateService {
    pub fn new(db_conn: &Arc<DatabasePool>) -> Self {
        Self {
            db_pool: Arc::clone(db_conn),
        }
    }

    /// Get random tweet templates for user selection
    pub async fn get_random_templates(
        &self,
        count: Option<i32>,
        category: Option<String>,
    ) -> Result<Vec<TweetTemplateResponse>, DatabaseError> {
        let pool = self.db_pool.get_pool();
        let limit = count.unwrap_or(5); // Default to 5 templates
        let category_ref = category.as_deref();

        TweetTemplateRepository::get_random_templates(pool, limit, category_ref).await
    }

    /// Get available categories
    pub async fn get_categories(&self) -> Result<Vec<String>, DatabaseError> {
        let pool = self.db_pool.get_pool();
        TweetTemplateRepository::get_categories(pool).await
    }

    /// Post tweet using template or custom content
    pub async fn post_tweet(
        &self,
        user_id: &str,
        request: PostTweetRequest,
    ) -> Result<PostTweetResponse, DatabaseError> {
        let pool = self.db_pool.get_pool();
        let content = if let Some(template_id) = request.template_id {
            // Use template
            let template = TweetTemplateRepository::get_by_id(pool, template_id).await?;
            match template {
                Some(t) => {
                    // Increment usage count
                    TweetTemplateRepository::increment_usage_count(pool, template_id).await?;
                    t.content
                }
                None => {
                    return Ok(PostTweetResponse {
                        success: false,
                        tweet_id: None,
                        message: "Template not found".to_string(),
                    });
                }
            }
        } else if let Some(custom) = request.custom_content {
            // Use custom content
            custom
        } else {
            return Ok(PostTweetResponse {
                success: false,
                tweet_id: None,
                message: "No content provided".to_string(),
            });
        };

        // Post to Twitter
        match self.post_to_twitter(&content, user_id).await {
            Ok(tweet_id) => Ok(PostTweetResponse {
                success: true,
                tweet_id: Some(tweet_id),
                message: "Tweet posted successfully!".to_string(),
            }),
            Err(e) => Ok(PostTweetResponse {
                success: false,
                tweet_id: None,
                message: format!("Failed to post tweet: {}", e),
            }),
        }
    }

    /// Post to Twitter using Twitter API v2
    /// Requires OAuth 2.0 user context tokens stored for the user
    async fn post_to_twitter(
        &self,
        content: &str,
        user_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Validate content length (Twitter's limit is 280 characters)
        if content.len() > 280 {
            return Err("Tweet content exceeds 280 character limit".into());
        }

        if content.trim().is_empty() {
            return Err("Tweet content cannot be empty".into());
        }

        // Get user's OAuth tokens from database
        let user_access_token = self.get_user_access_token(user_id).await?;

        // Create HTTP client with timeout
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        // Prepare the tweet payload
        let tweet_payload = json!({
            "text": content
        });

        // Make the API request
        let response = client
            .post("https://api.twitter.com/2/tweets")
            .header("Authorization", format!("Bearer {}", user_access_token))
            .header("Content-Type", "application/json")
            .json(&tweet_payload)
            .send()
            .await?;

        // Handle the response
        if response.status().is_success() {
            let tweet_data: Value = response.json().await?;

            // Extract tweet ID from response
            let tweet_id = tweet_data["data"]["id"]
                .as_str()
                .ok_or("No tweet ID found in API response")?;

            println!(
                "✅ Successfully posted tweet {} for user {}",
                tweet_id, user_id
            );
            Ok(tweet_id.to_string())
        } else {
            // Handle different error types
            let status = response.status();
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            match status.as_u16() {
                401 => Err("Twitter API authentication failed - invalid or expired tokens".into()),
                403 => Err(
                    "Twitter API forbidden - user may be suspended or app lacks permissions".into(),
                ),
                429 => Err("Twitter API rate limit exceeded - please try again later".into()),
                _ => Err(format!("Twitter API error {}: {}", status, error_body).into()),
            }
        }
    }

    /// Retrieve user's stored OAuth access token from database
    async fn get_user_access_token(&self, user_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let pool = self.db_pool.get_pool();
        
        let result = sqlx::query!(
            "SELECT twitter_access_token FROM users WHERE id = $1",
            uuid::Uuid::parse_str(user_id)?
        )
        .fetch_optional(pool)
        .await?;

        match result {
            Some(row) => {
                if let Some(token) = row.twitter_access_token {
                    Ok(token)
                } else {
                    Err(format!("No Twitter access token found for user {}", user_id).into())
                }
            }
            None => Err(format!("User {} not found", user_id).into())
        }
    }

    /// Alternative implementation for app-only authentication (read-only operations)
    #[allow(dead_code)]
    async fn post_to_twitter_app_only(content: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Note: App-only auth cannot post tweets, only read public data
        // This is kept for reference - actual posting requires user context

        let bearer_token =
            env::var("TWITTER_BEARER_TOKEN").map_err(|_| "Twitter Bearer Token not configured")?;

        Err("App-only authentication cannot post tweets - user context required".into())
    }

    /// Helper function to validate tweet content
    fn validate_tweet_content(content: &str) -> Result<(), &'static str> {
        if content.trim().is_empty() {
            return Err("Tweet content cannot be empty");
        }

        if content.len() > 280 {
            return Err("Tweet exceeds 280 character limit");
        }

        Ok(())
    }

    /// Get least used templates for better rotation
    pub async fn get_least_used_templates(
        &self,
        count: Option<i32>,
    ) -> Result<Vec<TweetTemplateResponse>, DatabaseError> {
        let pool = self.db_pool.get_pool();
        let limit = count.unwrap_or(5);
        TweetTemplateRepository::get_least_used_templates(pool, limit).await
    }
}
