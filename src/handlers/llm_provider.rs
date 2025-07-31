use axum::{
    extract::{State, Path, Extension},
    http::StatusCode,
    response::Json,
};
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    database::DbPool,
    models::{
        LlmProvider, NewLlmProvider, CreateLlmProviderRequest, 
        UpdateLlmProviderRequest, LlmProviderResponse
    },
    auth::Claims,
    schema::{llm_providers, llm_usage},
};

// Simple encryption for demo purposes - in production, use proper encryption
fn encrypt_api_key(api_key: &str) -> String {
    use base64::Engine;
    // This is a placeholder - implement proper encryption in production
    base64::engine::general_purpose::STANDARD.encode(api_key)
}

pub async fn create_llm_provider(
    Extension(claims): Extension<Claims>,
    State(pool): State<DbPool>,
    Json(request): Json<CreateLlmProviderRequest>,
) -> Result<Json<LlmProviderResponse>, StatusCode> {
    // Only admin can create LLM providers
    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let encrypted_api_key = encrypt_api_key(&request.api_key);

    let new_provider = NewLlmProvider {
        name: request.name,
        provider_type: request.provider_type,
        api_key_encrypted: encrypted_api_key,
        api_endpoint: request.api_endpoint,
        model_name: request.model_name,
    };

    let provider: LlmProvider = diesel::insert_into(llm_providers::table)
        .values(&new_provider)
        .returning(LlmProvider::as_select())
        .get_result(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(provider.into()))
}

pub async fn list_llm_providers(
    Extension(claims): Extension<Claims>,
    State(pool): State<DbPool>,
) -> Result<Json<Vec<LlmProviderResponse>>, StatusCode> {
    // Only admin can list LLM providers
    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let providers = llm_providers::table
        .select(LlmProvider::as_select())
        .load(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<LlmProviderResponse> = providers.into_iter().map(|p| p.into()).collect();
    Ok(Json(response))
}

pub async fn get_llm_provider(
    Extension(claims): Extension<Claims>,
    State(pool): State<DbPool>,
    Path(provider_id): Path<Uuid>,
) -> Result<Json<LlmProviderResponse>, StatusCode> {
    // Only admin can get LLM provider details
    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let provider = llm_providers::table
        .find(provider_id)
        .select(LlmProvider::as_select())
        .first(&mut conn)
        .optional()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(provider.into()))
}

pub async fn update_llm_provider(
    Extension(claims): Extension<Claims>,
    State(pool): State<DbPool>,
    Path(provider_id): Path<Uuid>,
    Json(request): Json<UpdateLlmProviderRequest>,
) -> Result<Json<LlmProviderResponse>, StatusCode> {
    // Only admin can update LLM providers
    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Simple update approach - update each field separately if provided
    if let Some(name) = &request.name {
        diesel::update(llm_providers::table.find(provider_id))
            .set(llm_providers::name.eq(name))
            .execute(&mut conn)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    if let Some(api_key) = &request.api_key {
        let encrypted_api_key = encrypt_api_key(api_key);
        diesel::update(llm_providers::table.find(provider_id))
            .set(llm_providers::api_key_encrypted.eq(encrypted_api_key))
            .execute(&mut conn)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    if let Some(api_endpoint) = &request.api_endpoint {
        diesel::update(llm_providers::table.find(provider_id))
            .set(llm_providers::api_endpoint.eq(api_endpoint))
            .execute(&mut conn)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    if let Some(model_name) = &request.model_name {
        diesel::update(llm_providers::table.find(provider_id))
            .set(llm_providers::model_name.eq(model_name))
            .execute(&mut conn)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    if let Some(is_active) = request.is_active {
        diesel::update(llm_providers::table.find(provider_id))
            .set(llm_providers::is_active.eq(is_active))
            .execute(&mut conn)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Update timestamp
    diesel::update(llm_providers::table.find(provider_id))
        .set(llm_providers::updated_at.eq(diesel::dsl::now))
        .execute(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch and return updated provider
    let updated_provider = llm_providers::table
        .find(provider_id)
        .select(LlmProvider::as_select())
        .first(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated_provider.into()))
}

pub async fn delete_llm_provider(
    Extension(claims): Extension<Claims>,
    State(pool): State<DbPool>,
    Path(provider_id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    // Only admin can delete LLM providers
    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let deleted_count = diesel::delete(llm_providers::table.find(provider_id))
        .execute(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if deleted_count == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_llm_usage_stats(
    Extension(claims): Extension<Claims>,
    State(pool): State<DbPool>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Only admin can view usage stats
    if claims.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut conn = pool.get().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Simple query to get basic usage stats
    let usage_count: i64 = llm_usage::table
        .count()
        .get_result(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let provider_count: i64 = llm_providers::table
        .count()
        .get_result(&mut conn)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = serde_json::json!({
        "total_providers": provider_count,
        "total_usage_records": usage_count,
        "message": "Detailed usage statistics implementation pending"
    });

    Ok(Json(response))
}
