use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at,
        }
    }
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::llm_providers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LlmProvider {
    pub id: Uuid,
    pub name: String,
    pub provider_type: String,
    pub api_key_encrypted: String,
    pub api_endpoint: Option<String>,
    pub model_name: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::llm_providers)]
pub struct NewLlmProvider {
    pub name: String,
    pub provider_type: String,
    pub api_key_encrypted: String,
    pub api_endpoint: Option<String>,
    pub model_name: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateLlmProviderRequest {
    pub name: String,
    pub provider_type: String,
    pub api_key: String,
    pub api_endpoint: Option<String>,
    pub model_name: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateLlmProviderRequest {
    pub name: Option<String>,
    pub api_key: Option<String>,
    pub api_endpoint: Option<String>,
    pub model_name: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Serialize)]
pub struct LlmProviderResponse {
    pub id: Uuid,
    pub name: String,
    pub provider_type: String,
    pub api_endpoint: Option<String>,
    pub model_name: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<LlmProvider> for LlmProviderResponse {
    fn from(provider: LlmProvider) -> Self {
        Self {
            id: provider.id,
            name: provider.name,
            provider_type: provider.provider_type,
            api_endpoint: provider.api_endpoint,
            model_name: provider.model_name,
            is_active: provider.is_active,
            created_at: provider.created_at,
            updated_at: provider.updated_at,
        }
    }
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::llm_usage)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct LlmUsage {
    pub id: Uuid,
    pub provider_id: Uuid,
    pub user_id: Uuid,
    pub tokens_used: i32,
    pub cost: Option<bigdecimal::BigDecimal>,
    pub request_type: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::llm_usage)]
pub struct NewLlmUsage {
    pub provider_id: Uuid,
    pub user_id: Uuid,
    pub tokens_used: i32,
    pub cost: Option<bigdecimal::BigDecimal>,
    pub request_type: String,
}
