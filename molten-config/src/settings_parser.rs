/// Runtime configuration parser

use secrecy::{ExposeSecret, SecretString};
use serde_aux::field_attributes::deserialize_number_from_string;
use sea_orm::ConnectOptions;
use crate::ConfigError;


/// Structure for all config settings
#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    /// Config settings for API
    pub application: AppSettings,
    /// Config settings for database
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct AppSettings {
    /// API host
    pub host: String,
    /// API port
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
}

/// Config struct to parse and store database configuration
#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    /// Database management system (current options: Postgres, MySQL)
    pub dbms: String, 
    /// Database username
    pub user: String,
    /// Database password
    pub password: SecretString,
    /// Database port
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    /// Database host address
    pub host: String,
    /// Database name
    pub database_name: String
}

impl DatabaseSettings {
    /// Configure the connection options for the SeaORM database
    pub fn get_connect_options(&self) -> ConnectOptions {
        ConnectOptions::new(format!(
            "{}://{}:{}@{}:{}/{}", // Valid for Postgres & MySQL
            &self.dbms,
            &self.user,
            &self.password.expose_secret(),
            &self.host,
            &self.port,
            &self.database_name
        ))
    }
}

/// Builds config from config files and env variables
pub fn get_configuration() -> Result<Settings, ConfigError> {
    // Set base path
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let config_dir = std::env::var("MOLTEN_CONFIG_DIR")
        .unwrap_or_else(|_| "config".to_string());
    let config_dir = base_path.join(config_dir);

    let settings = config::Config::builder()
        // Read app.{yaml|toml|json} file in config directory
        .add_source(config::File::from(config_dir.join("app")).required(true))
        // Add settings from environment variables with prefix MOLTEN and '__' separator 
        // e.g., MOLTEN_APPLICATION__PORT=1234 would overwrite `Settings.application.port` 
        .add_source(config::Environment::with_prefix("molten")
            .prefix_separator("_")
            .separator("__")
        )
        .build()?
        // Try to convert config values into Settings type
        .try_deserialize()?;

    Ok(settings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    fn setup_config_dir() -> TempDir {
        let tmp = TempDir::new().unwrap();

        let config_dir = tmp.path();

        fs::write(
            config_dir.join("app.yaml"),
            r#"
application:
  host: 127.0.0.1
  port: 8000

database:
  dbms: "postgres"
  host: "localhost"
  port: 5432
  user: "molten_user"
  password: "molten_password"
  database_name: "molten_db"
"#,
        )
        .unwrap();

        tmp
    }


    #[test]
    fn loads_settings_from_custom_config_dir() {
        // Arrange
        let config_dir = setup_config_dir();

        temp_env::with_var("MOLTEN_CONFIG_DIR", Some(config_dir.path()), || {
            // Act
            let settings = get_configuration().unwrap();

            // Assert
            assert_eq!(settings.application.port, 8000);
        });

    }

    #[test]
    fn overwrite_config_setting_with_env_var() {
        // Arrange
        let config_dir = setup_config_dir();

        temp_env::with_vars(
            [
                ("MOLTEN_CONFIG_DIR", Some(config_dir.path().to_str().unwrap())),
                ("MOLTEN_APPLICATION__PORT", Some("1234"))
            ], 
            || {
                            // Act
            let settings = get_configuration().unwrap();

            // Assert
            assert_eq!(settings.application.port, 1234);
            })
    }
}
