//! ตัวอย่าง Web API ด้วย Rust
//!
//! โปรเจกต์นี้แสดงการสร้าง REST API ด้วย Rust ใช้ axum framework
//! ครอบคลุม: HTTP routing, JSON handling, Error handling, Middleware, Testing

use anyhow::Result;
use axum::{
    Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

// Error types
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Internal server error")]
    Internal,
}

// Convert our error type to HTTP response
impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User not found".to_string()),
            Self::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(serde_json::json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

type ApiResult<T> = Result<T, ApiError>;

// Data models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub age: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub age: u32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct UserQuery {
    pub name: Option<String>,
    pub min_age: Option<u32>,
    pub max_age: Option<u32>,
}

// Application state
#[derive(Debug, Clone)]
pub struct AppState {
    pub users: Arc<Mutex<HashMap<Uuid, User>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

impl AppState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

// API handlers
pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "version": "1.0.0"
    }))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> ApiResult<(StatusCode, Json<User>)> {
    // Validate input
    if payload.name.trim().is_empty() {
        return Err(ApiError::InvalidInput("Name cannot be empty".to_string()));
    }

    if !payload.email.contains('@') {
        return Err(ApiError::InvalidInput("Invalid email format".to_string()));
    }

    if payload.age > 150 {
        return Err(ApiError::InvalidInput("Age must be realistic".to_string()));
    }

    let user = User {
        id: Uuid::new_v4(),
        name: payload.name,
        email: payload.email,
        age: payload.age,
        created_at: chrono::Utc::now(),
    };

    let mut users = state.users.lock().map_err(|_| ApiError::Internal)?;
    users.insert(user.id, user.clone());

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_users(
    State(state): State<AppState>,
    Query(params): Query<UserQuery>,
) -> ApiResult<Json<Vec<User>>> {
    let users = state.users.lock().map_err(|_| ApiError::Internal)?;

    let filtered_users: Vec<User> = users
        .values()
        .filter(|user| {
            // Filter by name if provided
            if let Some(ref name_filter) = params.name {
                if !user
                    .name
                    .to_lowercase()
                    .contains(&name_filter.to_lowercase())
                {
                    return false;
                }
            }

            // Filter by min age if provided
            if let Some(min_age) = params.min_age {
                if user.age < min_age {
                    return false;
                }
            }

            // Filter by max age if provided
            if let Some(max_age) = params.max_age {
                if user.age > max_age {
                    return false;
                }
            }

            true
        })
        .cloned()
        .collect();

    Ok(Json(filtered_users))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<Json<User>> {
    let users = state.users.lock().map_err(|_| ApiError::Internal)?;

    let user = users.get(&user_id).ok_or(ApiError::UserNotFound)?.clone();

    Ok(Json(user))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> ApiResult<Json<User>> {
    let mut users = state.users.lock().map_err(|_| ApiError::Internal)?;

    let user = users.get_mut(&user_id).ok_or(ApiError::UserNotFound)?;

    // Update fields if provided
    if let Some(name) = payload.name {
        if name.trim().is_empty() {
            return Err(ApiError::InvalidInput("Name cannot be empty".to_string()));
        }
        user.name = name;
    }

    if let Some(email) = payload.email {
        if !email.contains('@') {
            return Err(ApiError::InvalidInput("Invalid email format".to_string()));
        }
        user.email = email;
    }

    if let Some(age) = payload.age {
        if age > 150 {
            return Err(ApiError::InvalidInput("Age must be realistic".to_string()));
        }
        user.age = age;
    }

    Ok(Json(user.clone()))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let mut users = state.users.lock().map_err(|_| ApiError::Internal)?;

    users.remove(&user_id).ok_or(ApiError::UserNotFound)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_user_stats(State(state): State<AppState>) -> ApiResult<Json<serde_json::Value>> {
    let users = state.users.lock().map_err(|_| ApiError::Internal)?;

    let total_users = users.len();
    let average_age = if total_users > 0 {
        users.values().map(|u| f64::from(u.age)).sum::<f64>() / total_users as f64
    } else {
        0.0
    };

    let age_groups = users
        .values()
        .fold(HashMap::new(), |mut acc: HashMap<String, u32>, user| {
            let group = match user.age {
                0..=17 => "0-17",
                18..=25 => "18-25",
                26..=35 => "26-35",
                36..=50 => "36-50",
                _ => "50+",
            };
            *acc.entry(group.to_string()).or_insert(0) += 1;
            acc
        });

    Ok(Json(serde_json::json!({
        "total_users": total_users,
        "average_age": average_age,
        "age_groups": age_groups
    })))
}

// Create the router
pub fn create_app() -> Router {
    let state = AppState::new();

    Router::new()
        .route("/health", get(health_check))
        .route("/users", get(get_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        .route("/users/stats", get(get_user_stats))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .with_state(state)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = create_app();

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Server running on http://localhost:3000");
    println!("📋 API Endpoints:");
    println!("  GET    /health        - Health check");
    println!("  GET    /users         - Get all users (with optional filters)");
    println!("  POST   /users         - Create new user");
    println!("  GET    /users/:id     - Get user by ID");
    println!("  PUT    /users/:id     - Update user");
    println!("  DELETE /users/:id     - Delete user");
    println!("  GET    /users/stats   - Get user statistics");

    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_user() {
        let app = create_app();

        let user_data = serde_json::json!({
            "name": "John Doe",
            "email": "john@example.com",
            "age": 30
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header("content-type", "application/json")
                    .body(Body::from(user_data.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_create_user_invalid_email() {
        let app = create_app();

        let user_data = serde_json::json!({
            "name": "John Doe",
            "email": "invalid-email",
            "age": 30
        });

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header("content-type", "application/json")
                    .body(Body::from(user_data.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_nonexistent_user() {
        let app = create_app();
        let fake_id = Uuid::new_v4();

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&format!("/users/{}", fake_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_user_stats_empty() {
        let app = create_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users/stats")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let stats: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(stats["total_users"], 0);
        assert_eq!(stats["average_age"], 0.0);
    }
}

// Integration tests
#[cfg(test)]
mod integration_tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_full_user_lifecycle() {
        let app = create_app();

        // Create user
        let user_data = serde_json::json!({
            "name": "Alice Smith",
            "email": "alice@example.com",
            "age": 25
        });

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header("content-type", "application/json")
                    .body(Body::from(user_data.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let created_user: User = serde_json::from_slice(&body).unwrap();
        let user_id = created_user.id;

        // Get user
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(&format!("/users/{}", user_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Update user
        let update_data = serde_json::json!({
            "name": "Alice Johnson",
            "age": 26
        });

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(&format!("/users/{}", user_id))
                    .header("content-type", "application/json")
                    .body(Body::from(update_data.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Delete user
        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(&format!("/users/{}", user_id))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
