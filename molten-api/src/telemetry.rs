use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    // Function is generic for MakeWriter trait allowing us to choose
    // where messages are written to (e.g., std::io::{stdout, sink} )
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env()
        // If no RUST_LOG env variable is set, set the Env Filter manually
        .or_else(|_| EnvFilter::try_new(env_filter))
        .expect("Unable to set logging level.");
    let formatting_layer = tracing_subscriber::fmt::layer();

    Registry::default().with(env_filter).with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect `log`'s events to the subscriber
    LogTracer::init().expect("Failed to set logger");
    // set_global_default specifies the subscriber used to process spans
    set_global_default(subscriber).expect("Failed to set subscriber");
}
