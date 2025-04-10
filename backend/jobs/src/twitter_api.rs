use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub text: String,
    pub created_at: String,
    pub author_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitterResponse {
    pub data: Vec<Tweet>,
    pub meta: Option<TwitterMeta>,
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
        since_id: &Option<String>,
        max_results: Option<usize>,
    ) -> Result<TwitterResponse, Box<dyn Error>> {
        // Build the search query
        let query = format!("@{} #{}", username, hashtag);
        let mut url = format!(
            "https://api.twitter.com/2/tweets/search/recent?query={}&sort_order=relevancy&tweet.fields=created_at,author_id",
            urlencoding::encode(&query)
        );

        // Add parameters
        if let Some(id) = since_id {
            url = format!("{}&since_id={}", url, id);
        }
        if let Some(n) = max_results {
            url = format!("{}&max_results={}", url, n);
        }

        // Prepare headers
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.bearer_token))?,
        );

        // Make the request
        let response = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await?
            .json::<TwitterResponse>()
            .await?;

        Ok(response)
    }

    pub async fn get_all_tweets(
        &self,
        username: &str,
        hashtag: &str,
        initial_since_id: &Option<String>,
        max_results_per_request: usize,
    ) -> Result<Vec<Tweet>, Box<dyn Error>> {
        let mut all_tweets = Vec::new();
        let mut since_id = initial_since_id.clone();

        loop {
            let response = self
                .get_tweets(username, hashtag, &since_id, Some(max_results_per_request))
                .await?;

            if let Some(meta) = response.meta {
                since_id = meta.oldest_id;
            } else {
                break;
            }

            all_tweets.extend(response.data);
        }

        Ok(all_tweets)
    }
}
