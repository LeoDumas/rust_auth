use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{net::SocketAddr, sync::Arc};
use dotenv::dotenv;

mod utils;
use utils::jwt_utils;


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: i32,
    username: String,
    email: String,
    password: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
struct LoginRequest{
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
struct UserResponse {
    id: i32,
    username: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Create database pool
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Create users table if not exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            email VARCHAR(255) NOT NULL UNIQUE,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/users", post(create_user))
        .route("/users", get(get_users))
        .route("/auth/login", post(login))
        .with_state(Arc::new(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn hello_world() -> &'static str {
    "Hello people!"
}

async fn create_user(
    State(pool): State<Arc<Pool<Postgres>>>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let hashed_password = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email, password)
        VALUES ($1, $2, $3)
        RETURNING id, username, email, password, created_at
        "#,
    )
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(hashed_password)
    .fetch_one(&*pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create user: {}", e),
        )
    })?;

    Ok(Json(user))
}

async fn verify_password(
    stored_hash: &str,
    attempted_password: &str
) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(attempted_password, stored_hash)
}

async fn login(
    State(pool): State<Arc<Pool<Postgres>>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl axum::response::IntoResponse, (StatusCode, String)> {
    // Fetch the user by email
    let user = sqlx::query_as::<_, User>(
        r#"SELECT id, email, username, password, created_at FROM users WHERE email = $1"#,
    )
    .bind(&payload.email)
    .fetch_optional(&*pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = user.ok_or_else(|| {
        // Use a generic error message to prevent user enumeration
        (StatusCode::UNAUTHORIZED, "Invalid email or password".to_string())
    })?;

    // Verify the password
    let is_valid = verify_password(&user.password, &payload.password)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !is_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid email or password".to_string()));
    }

    // Generate JWT token
    let token = jwt_utils::generate_token(user.id, user.email.clone(), user.username.clone())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "token": token,
        "user_id": user.id,
        "user_email": user.email,
        "user_username": user.username,
    })))
}

async fn get_users(State(pool): State<Arc<Pool<Postgres>>>) -> Result<Json<Vec<UserResponse>>, (StatusCode, String)> {
    let users = sqlx::query_as::<_, UserResponse>(
        r#"
        SELECT id, username, email, created_at
        FROM users
        "#,
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch users: {}", e),
        )
    })?;

    Ok(Json(users))
}