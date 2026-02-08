//! This module provides utilities for setting up application-wide logging and tracing.
//!
//! It leverages the `tracing` and `tracing-subscriber` crates to configure
//! how log events and spans are collected and emitted, allowing for better
//! observability of the Molten API.
use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};

/// Returns a `Subscriber` that can be used for logging and tracing.
///
/// This function configures an `EnvFilter` to filter log events based on
/// environment variables or a default filter string, and sets up a `fmt` layer
/// for formatting and outputting log records to a specified sink.
///
/// # Arguments
/// * `env_filter` - A string defining the default logging level and filter directives.
/// * `sink` - A type that implements `MakeWriter` trait, determining where the logs are written (e.g., `stdout`, `stderr`, a file).
///
/// # Type Parameters
/// * `Sink` - A type that can create `std::io::Write` instances, enabling flexible log output.
///
/// # Returns
/// An `impl Subscriber + Send + Sync` configured with the specified filter and sink.
pub fn get_subscriber<Sink>(env_filter: String, sink: Sink) -> impl Subscriber + Send + Sync
where
    // Function is generic for MakeWriter trait allowing us to choose
    // where messages are written to (e.g., std::io::{stdout, sink} )
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter = EnvFilter::try_from_default_env()
        // If no RUST_LOG env variable is set, set the Env Filter manually
        .or_else(|_| EnvFilter::try_new(env_filter))
        .expect("Unable to set logging level.");
    let formatting_layer = tracing_subscriber::fmt::layer().with_writer(sink);

    Registry::default().with(env_filter).with(formatting_layer)
}

/// Initializes the global default tracing subscriber.
///
/// This function sets the provided `subscriber` as the global default for processing
/// `tracing` spans and events. It also redirects `log` crate events to this subscriber.
///
/// # Arguments
/// * `subscriber` - An `impl Subscriber + Send + Sync` which will be set as the global default.
///
/// # Panics
/// Panics if unable to set the global logger or subscriber, typically indicating
/// that a subscriber has already been set.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect `log`'s events to the subscriber
    LogTracer::init().expect("Failed to set logger");
    // set_global_default specifies the subscriber used to process spans
    set_global_default(subscriber).expect("Failed to set subscriber");
}
