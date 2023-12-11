use tokio::task::JoinHandle;
use tracing::Subscriber;

use tracing_subscriber::{fmt::MakeWriter, EnvFilter};

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    let _ = tracing::subscriber::set_global_default(subscriber)
        .map_err(|_err| eprintln!("Unable to set global default subscriber"));
}

/// Calls tokio::task::spawn_blocking() with a tracing span wrapped around it.
pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}

/// Sets up the tracing subscriber.
#[cfg(not(feature = "bunyan"))]
pub fn get_subscriber<Sink>(
    _name: &str,
    env_filter: &str,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    use tracing_subscriber::{
        fmt::{self, format::FmtSpan},
        prelude::*,
    };

    let filter_layer =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    // --This code uses tracing-subscriber--
    let fmt_layer = fmt::layer()
        .compact()
        .with_target(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::ACTIVE)
        .with_writer(sink);

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
    //----
}

/// Sets up the tracing subscriber.
#[cfg(feature = "bunyan")]
pub fn get_subscriber<Sink>(
    name: &str,
    env_filter: &str,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
    use tracing_subscriber::{layer::SubscriberExt, Registry};

    let filter_layer =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    // --This code uses bunyan --
    let bunyan_format = BunyanFormattingLayer::new(name.to_string(), sink);

    Registry::default()
        .with(filter_layer)
        .with(JsonStorageLayer)
        .with(bunyan_format)
}
