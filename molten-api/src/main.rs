use molten_api::create_app;
use molten_migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Initialize Logging
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 2. Connect to Database
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url).await?;
    tracing::info!("Connected to database: {}", db_url);

    // 3. Run migrations
    tracing::info!("Running database migrations...");
    Migrator::up(&db, None).await?;
    tracing::info!("Migrations applied successfully.");

    // 4. Create the App (Router)
    let app = create_app(db);

    // 5. Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
