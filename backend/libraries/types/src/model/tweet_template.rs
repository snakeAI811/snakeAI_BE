use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TweetTemplate {
    pub id: i32,
    pub content: String,
    pub category: Option<String>,
    pub is_active: bool,
    pub usage_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TweetTemplateResponse {
    pub id: i32,
    pub content: String,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTweetRequest {
    pub template_id: Option<i32>,
    pub custom_content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTweetResponse {
    pub success: bool,
    pub tweet_id: Option<String>,
    pub message: String,
}