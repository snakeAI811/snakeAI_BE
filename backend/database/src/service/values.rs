use crate::repository::values::ValuesRepository;
use types::error::{ApiError, DbError};

#[derive(Clone)]
pub struct ValuesService {
    values_repo: ValuesRepository,
}

impl ValuesService {
    pub fn new(values_repo: ValuesRepository) -> Self {
        Self { values_repo }
    }

    pub async fn set_value(&self, key: &str, value: &str) -> Result<(), ApiError> {
        self.values_repo
            .set_value(key, value)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_value(&self, key: &str) -> Result<Option<String>, ApiError> {
        self.values_repo
            .get_value(key)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn delete_value(&self, key: &str) -> Result<bool, ApiError> {
        self.values_repo
            .delete_value(key)
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    pub async fn get_all_values(&self) -> Result<Vec<(String, String)>, ApiError> {
        self.values_repo
            .get_all_values()
            .await
            .map_err(|err| DbError::SomethingWentWrong(err.to_string()).into())
    }

    // TCE specific methods
    pub async fn set_tce_status(&self, started: bool) -> Result<(), ApiError> {
        self.set_value("tce_started", &started.to_string()).await
    }

    pub async fn get_tce_status(&self) -> Result<bool, ApiError> {
        match self.get_value("tce_started").await? {
            Some(value) => Ok(value.parse().unwrap_or(false)),
            None => Ok(false), // Default to false if not set
        }
    }
}