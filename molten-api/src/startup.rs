//! This module handles the application startup process for the Molten API.
//!
//! It includes logic for connecting to the database, running migrations,
//! building the Axum application, and starting the HTTP server.
use tokio::net::TcpListener;

use crate::handlers;
use crate::{error::BuildError, state::AppState};
use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use molten_config::settings_parser::Settings;
use molten_migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};

/// Represents the Molten API application, encapsulating the server's network listener,
/// application-wide state, and the port it is bound to.
pub struct Application {
    listener: TcpListener,
    state: AppState,
    port: u16,
}

impl Application {
    /// Builds a new `Application` instance by connecting to the database,
    /// running pending migrations, and setting up the TCP listener.
    ///
    /// # Arguments
    /// * `config` - The application settings loaded from configuration.
    ///
    /// # Returns
    /// A `Result` which is `Ok` with the `Application` instance if successful,
    /// or `Err` with a `BuildError` if any step fails.
    pub async fn build(config: Settings) -> Result<Self, BuildError> {
        // Connect to Database
        let db: DatabaseConnection = Self::get_db_connection(&config).await?;
        tracing::info!("Connected to database: {}", &config.database.database_name);

        // Run migrations
        Self::run_migrations(&db).await?;

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

    /// Establishes a database connection using the provided application settings.
    ///
    /// # Arguments
    /// * `config` - A reference to the application settings.
    ///
    /// # Returns
    /// A `Result` which is `Ok` with the `DatabaseConnection` if successful,
    /// or `Err` with a `DbErr` if the connection fails.
    async fn get_db_connection(config: &Settings) -> Result<DatabaseConnection, DbErr> {
        Database::connect(config.database.get_connect_options()).await
    }

    /// Runs any pending database migrations.
    ///
    /// # Arguments
    /// * `db` - A reference to the database connection.
    ///
    /// # Returns
    /// A `Result` which is `Ok` if migrations are applied successfully,
    /// or `Err` with a `DbErr` if an error occurs during migration.
    async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
        tracing::info!("Running database migrations...");
        Migrator::up(db, None).await?;
        tracing::info!("Migrations applied successfully.");
        Ok(())
    }

    /// Creates the Axum router with all routes and state attached.
    ///
    /// # Arguments
    /// * `db` - The database connection
    ///
    /// # Returns
    /// An axum Router
    fn define_router(db: DatabaseConnection) -> Router {
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

    /// Runs the application
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
