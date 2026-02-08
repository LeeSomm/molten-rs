use molten_api::startup::Application;
use molten_api::telemetry::{get_subscriber, init_subscriber};
use molten_config::settings_parser::get_configuration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Initialize Environment Variables & Logging
    dotenvy::dotenv().ok();
    let subscriber = get_subscriber(
        "molten-api".into(),
        "molten-api=info,tower-http=info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    // 2. Get configuration settings
    let config = get_configuration().expect("Failed to parse configuration settings");

    // 3. Create the App
    let app = Application::build(config).await?;

    // 4. Start Server
    app.run().await?;

    Ok(())
}
