mod config;
mod models;
mod schema;

use axum::{
    routing::{get, post},
    Router,
};
use axum::serve;
use config::Config;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Configuration loaded successfully");

    // Initialize database connection pool
    let manager = ConnectionManager::<PgConnection>::new(&config.database.url);
    let pool = Pool::builder()
        .max_size(config.database.max_connections)
        .build(manager)?;
    tracing::info!("Database connection pool initialized");

    // Run migrations
    let mut conn = pool.get()?;
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Migration failed: {}", e))?;
    tracing::info!("Database migrations completed");

    // Create application state
    let state = Arc::new(AppState {
        pool,
        config: config.clone(),
    });

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/track", post(track_event))
        .route("/consent", post(update_consent))
        .with_state(state);

    // Start server
    tracing::info!("Starting server on {}", config.server.address);
    let listener = tokio::net::TcpListener::bind(config.server.address).await?;
    serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
struct AppState {
    pool: Pool<ConnectionManager<PgConnection>>,
    config: Config,
}

async fn health_check() -> &'static str {
    "OK"
}

async fn track_event() -> &'static str {
    "Event tracked"
}

async fn update_consent() -> &'static str {
    "Consent updated"
} 
