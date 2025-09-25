use chrono::{DateTime, Utc};
use sqlx::PgPool;
use types::model::{TweetTemplate, TweetTemplateResponse};
use crate::error::DatabaseError;

pub struct TweetTemplateRepository;

impl TweetTemplateRepository {
    /// Get random tweet templates for user selection
    pub async fn get_random_templates(
        pool: &PgPool,
        limit: i32,
        category: Option<&str>,
    ) -> Result<Vec<TweetTemplateResponse>, DatabaseError> {
        let limit = limit as i64;
        
        let templates = if let Some(cat) = category {
            sqlx::query_as!(
                TweetTemplate,
                r#"
                SELECT 
                    id, 
                    content, 
                    category, 
                    is_active as "is_active!: bool", 
                    usage_count as "usage_count!: i32", 
                    created_at as "created_at!: DateTime<Utc>", 
                    updated_at as "updated_at!: DateTime<Utc>"
                FROM tweet_templates 
                WHERE is_active = true AND category = $1
                ORDER BY RANDOM() 
                LIMIT $2
                "#,
                cat,
                limit
            )
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as!(
                TweetTemplate,
                r#"
                SELECT 
                    id, 
                    content, 
                    category, 
                    is_active as "is_active!: bool", 
                    usage_count as "usage_count!: i32", 
                    created_at as "created_at!: DateTime<Utc>", 
                    updated_at as "updated_at!: DateTime<Utc>"
                FROM tweet_templates 
                WHERE is_active = true 
                ORDER BY RANDOM() 
                LIMIT $1
                "#,
                limit
            )
            .fetch_all(pool)
            .await?
        };
        
        let response: Vec<TweetTemplateResponse> = templates
            .into_iter()
            .map(|t| TweetTemplateResponse {
                id: t.id,
                content: t.content,
                category: t.category,
            })
            .collect();

        Ok(response)
    }

    /// Get all available categories
    pub async fn get_categories(pool: &PgPool) -> Result<Vec<String>, DatabaseError> {
        let categories = sqlx::query_scalar!(
            r#"
            SELECT DISTINCT category 
            FROM tweet_templates 
            WHERE is_active = true AND category IS NOT NULL
            ORDER BY category
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(categories.into_iter().filter_map(|c| c).collect())
    }

    /// Increment usage count for a template
    pub async fn increment_usage_count(
        pool: &PgPool,
        template_id: i32,
    ) -> Result<(), DatabaseError> {
        sqlx::query!(
            r#"
            UPDATE tweet_templates 
            SET usage_count = usage_count + 1, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
            template_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Get template by ID
    pub async fn get_by_id(
        pool: &PgPool,
        template_id: i32,
    ) -> Result<Option<TweetTemplate>, DatabaseError> {
        let template = sqlx::query_as!(
            TweetTemplate,
            r#"
            SELECT 
                id, 
                content, 
                category, 
                is_active as "is_active!: bool", 
                usage_count as "usage_count!: i32", 
                created_at as "created_at!: DateTime<Utc>", 
                updated_at as "updated_at!: DateTime<Utc>"
            FROM tweet_templates 
            WHERE id = $1 AND is_active = true
            "#,
            template_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(template)
    }

    /// Get templates with least usage (for rotation)
    pub async fn get_least_used_templates(
        pool: &PgPool,
        limit: i32,
    ) -> Result<Vec<TweetTemplateResponse>, DatabaseError> {
        let limit = limit as i64;
        let templates = sqlx::query_as!(
            TweetTemplate,
            r#"
            SELECT 
                id, 
                content, 
                category, 
                is_active as "is_active!: bool", 
                usage_count as "usage_count!: i32", 
                created_at as "created_at!: DateTime<Utc>", 
                updated_at as "updated_at!: DateTime<Utc>"
            FROM tweet_templates 
            WHERE is_active = true 
            ORDER BY usage_count ASC, RANDOM()
            LIMIT $1
            "#,
            limit
        )
        .fetch_all(pool)
        .await?;

        let response: Vec<TweetTemplateResponse> = templates
            .into_iter()
            .map(|t| TweetTemplateResponse {
                id: t.id,
                content: t.content,
                category: t.category,
            })
            .collect();

        Ok(response)
    }

    /// Add new template (for admin use)
    pub async fn create_template(
        pool: &PgPool,
        content: &str,
        category: Option<&str>,
    ) -> Result<TweetTemplate, DatabaseError> {
        let template = sqlx::query_as!(
            TweetTemplate,
            r#"
            INSERT INTO tweet_templates (content, category)
            VALUES ($1, $2)
            RETURNING 
                id, 
                content, 
                category, 
                is_active as "is_active!: bool", 
                usage_count as "usage_count!: i32", 
                created_at as "created_at!: DateTime<Utc>", 
                updated_at as "updated_at!: DateTime<Utc>"
            "#,
            content,
            category
        )
        .fetch_one(pool)
        .await?;

        Ok(template)
    }

    /// Deactivate template (soft delete)
    pub async fn deactivate_template(
        pool: &PgPool,
        template_id: i32,
    ) -> Result<(), DatabaseError> {
        sqlx::query!(
            r#"
            UPDATE tweet_templates 
            SET is_active = false, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
            template_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}