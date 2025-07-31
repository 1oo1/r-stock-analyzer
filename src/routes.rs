use axum::{
    routing::{get, post, put, delete},
    Router,
    middleware,
};

use crate::{
    database::DbPool,
    handlers::{user, llm_provider},
    middleware::{auth_middleware, admin_middleware},
};

pub fn create_routes(pool: DbPool) -> Router {
    Router::new()
        // Public routes
        .route("/api/auth/register", post(user::register_user))
        .route("/api/auth/login", post(user::login_user))
        
        // Protected user routes
        .route("/api/user/me", get(user::get_current_user))
        .route_layer(middleware::from_fn(auth_middleware))
        
        // Admin-only routes for user management
        .route("/api/admin/users", get(user::list_users))
        .route_layer(middleware::from_fn(admin_middleware))
        
        // Admin-only routes for LLM provider management
        .route("/api/admin/llm-providers", post(llm_provider::create_llm_provider))
        .route("/api/admin/llm-providers", get(llm_provider::list_llm_providers))
        .route("/api/admin/llm-providers/:id", get(llm_provider::get_llm_provider))
        .route("/api/admin/llm-providers/:id", put(llm_provider::update_llm_provider))
        .route("/api/admin/llm-providers/:id", delete(llm_provider::delete_llm_provider))
        .route("/api/admin/llm-usage-stats", get(llm_provider::get_llm_usage_stats))
        .route_layer(middleware::from_fn(admin_middleware))
        
        .with_state(pool)
}
