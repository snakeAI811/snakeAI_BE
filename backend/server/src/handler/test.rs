use axum::Json;
use serde_json::json;

// Test handlers for debugging the API
pub async fn test_profile() -> Json<serde_json::Value> {
    Json(json!({
        "role": "None",
        "status": "active"
    }))
}

pub async fn test_token_info() -> Json<serde_json::Value> {
    Json(json!({
        "balance": 1000,
        "locked": 0,
        "staked": 0,
        "rewards": 50
    }))
}

pub async fn test_patron_application() -> Json<serde_json::Value> {
    Json(json!({
        "id": "test-123",
        "status": "pending",
        "submitted_at": "2025-01-01T00:00:00Z"
    }))
}

pub async fn test_active_swaps() -> Json<serde_json::Value> {
    Json(json!([]))
}

pub async fn test_my_swaps() -> Json<serde_json::Value> {
    Json(json!([]))
}

pub async fn test_vesting_info() -> Json<serde_json::Value> {
    Json(json!({
        "amount": 0,
        "role_type": "None",
        "created_at": "2025-01-01T00:00:00Z",
        "withdrawal_available": false
    }))
}

// Generate a test session token for development testing
pub async fn test_auth_token() -> Json<serde_json::Value> {
    Json(json!({
        "session_token": "test-session-12345",
        "user_id": "test-user-67890",
        "expires_in": 3600,
        "note": "This is a test token for development only. Use real OAuth for production."
    }))
}
