use tokio::net::TcpListener;

use crate::state::AppState;
use crate::handlers;
use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use molten_config::settings_parser::{Settings, get_configuration};
use molten_migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};

pub struct Application {
    listener: TcpListener,
    state: AppState,
    port: u16,
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, Error> {
        //TODO: Make custom error
        // Get configuration settings
        let config = get_configuration().expect("Failed to parse configuration settings");

        // Connect to Database
        let db: DatabaseConnection = Self::get_db_connection(&config).await?;
        tracing::info!("Connected to database: {}", &config.database.database_name);

        // Run migrations
        Self::run_migrations(&db);

        let state = AppState::new(db);
        let addr = format!("{}:{}", config.application.host, config.application.port);
        tracing::info!("Listening on {}", addr);
        let listener = TcpListener::bind(addr).await?;
        let port = listener.local_addr().unwrap().port();

        // Return Application
        Ok(Self {
            listener,
            state,
            port,
        })
    }

    // This is useful because when the port in config is 0, a random port will be assigned
    // which we need to know post hoc.
    /// Port number getter
    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn get_db_connection(config: &Settings) -> Result<DatabaseConnection, DbErr> {
        Database::connect(config.database.get_connect_options()).await
    }

    pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
        tracing::info!("Running database migrations...");
        Migrator::up(db, None).await?;
        tracing::info!("Migrations applied successfully.");
        Ok(())
    }

    /// Creates the Axum router with all routes and state attached.
    /// This function is now testable without starting a real TCP listener.
    pub fn define_router(db: DatabaseConnection) -> Router {
        let state = AppState::new(db);

        Router::new()
            .route("/health", get(|| async { StatusCode::OK }))
            .route("/documents", post(handlers::create_document))
            .route("/documents/{id}", get(handlers::get_document))
            .route("/forms", post(handlers::create_form))
            .route("/forms/{id}", get(handlers::get_form))
            .route("/workflows", post(handlers::create_workflow))
            .route("/workflows/{id}", get(handlers::get_workflow))
            .with_state(state)
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        let Application {
            listener,
            state,
            port: _,
        } = self;
        let router = Self::define_router(state.db);
        axum::serve(listener, router.into_make_service()).await
    }
}
