use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    Extension,
};
use serde::{Deserialize, Serialize};
use types::model::{TweetTemplateResponse, PostTweetRequest, PostTweetResponse, User};
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct GetTemplatesQuery {
    pub count: Option<i32>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetTemplatesResponse {
    pub templates: Vec<TweetTemplateResponse>,
    pub categories: Vec<String>,
}

/// Get random tweet templates for user selection
pub async fn get_tweet_templates(
    State(state): State<AppState>,
    Extension(_user): Extension<User>,
    Query(params): Query<GetTemplatesQuery>,
) -> Result<Json<GetTemplatesResponse>, StatusCode> {
    // Get templates
    let templates = state.service.tweet_template.get_random_templates(
        params.count,
        params.category,
    )
    .await
    .map_err(|e| {
        log::error!("Failed to get tweet templates: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get available categories
    let categories = state.service.tweet_template.get_categories()
        .await
        .map_err(|e| {
            log::error!("Failed to get categories: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(GetTemplatesResponse {
        templates,
        categories,
    }))
}

/// Post a tweet using template or custom content
pub async fn post_tweet(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Json(request): Json<PostTweetRequest>,
) -> Result<Json<PostTweetResponse>, StatusCode> {
    let response = state.service.tweet_template.post_tweet(
        &user.id.to_string(),
        request,
    )
    .await
    .map_err(|e| {
        log::error!("Failed to post tweet: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(response))
}

/// Get least used templates for better rotation
pub async fn get_fresh_templates(
    State(state): State<AppState>,
    Extension(_user): Extension<User>,
    Query(params): Query<GetTemplatesQuery>,
) -> Result<Json<Vec<TweetTemplateResponse>>, StatusCode> {
    let templates = state.service.tweet_template.get_least_used_templates(
        params.count,
    )
    .await
    .map_err(|e| {
        log::error!("Failed to get fresh templates: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(templates))
}