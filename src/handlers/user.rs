use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::Json,
};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    auth::{create_jwt, hash_password, verify_password, Claims},
    database::DbPool,
    models::{CreateUserRequest, LoginRequest, LoginResponse, NewUser, User, UserResponse},
    schema::users,
};

pub async fn register_user(
    State(pool): State<DbPool>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if user already exists
    let existing_user = users::table
        .filter(users::username.eq(&request.username))
        .or_filter(users::email.eq(&request.email))
        .select(User::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing_user.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    // Hash password
    let password_hash =
        hash_password(&request.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create new user
    let new_user = NewUser {
        username: request.username,
        email: request.email,
        password_hash,
        role: "user".to_string(), // Default role
    };

    let user: User = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_select())
        .get_result(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user.into()))
}

pub async fn login_user(
    State(pool): State<DbPool>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Find user
    let user = users::table
        .filter(users::username.eq(&request.username))
        .filter(users::is_active.eq(true))
        .select(User::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify password
    let is_valid = verify_password(&request.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !is_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Create JWT token
    let token = create_jwt(user.id, &user.username, &user.role)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        token,
        user: user.into(),
    }))
}

pub async fn get_current_user(
    Extension(claims): Extension<Claims>,
    State(pool): State<DbPool>,
) -> Result<Json<UserResponse>, StatusCode> {
    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = users::table
        .find(user_id)
        .select(User::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(user.into()))
}

pub async fn list_users(
    Extension(claims): Extension<Claims>,
    State(pool): State<DbPool>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    // Only admin can list users
    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let users_list = users::table
        .select(User::as_select())
        .load(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<UserResponse> = users_list.into_iter().map(|u| u.into()).collect();
    Ok(Json(response))
}
