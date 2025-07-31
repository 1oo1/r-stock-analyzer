mod auth;
mod database;
mod environments;
mod handlers;
mod middleware;
mod models;
mod routes;
mod schema;

use axum::Router;
use diesel_migrations::{FileBasedMigrations, MigrationHarness};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{self, fmt, EnvFilter};

#[tokio::main]
async fn main() {
    // Load environment variables
    environments::init_env();
    let is_development = environments::is_development();

    // Initialize log tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap())
        .with(fmt::layer().with_target(false))
        .init();

    // Create database connection pool
    let pool = database::create_connection_pool();

    // Run migrations (in production)
    if !is_development {
        let conn = &mut pool.get().unwrap();
        match conn.run_pending_migrations(FileBasedMigrations::find_migrations_directory().unwrap())
        {
            Ok(_) => tracing::info!("Migrations executed successfully"),
            Err(e) => {
                tracing::error!("Failed to run migrations: {}", e);
                return;
            }
        }
    }

    // Create application router
    let mut app = Router::new().merge(routes::create_routes(pool));
    if is_development {
        // Enable CORS in development mode, permitting all origins
        app = app.layer(CorsLayer::permissive());
    }

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
