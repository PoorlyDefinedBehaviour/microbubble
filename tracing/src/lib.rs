use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use tracing_tree::HierarchicalLayer;

pub use tracing::{debug, error, info, instrument, warn};

/// Initializes the tracing subscriber. Meant to be called on main by every package that needs tracing or logging.
pub fn init() {
    let subscriber = Registry::default().with(HierarchicalLayer::new(1)).with(
        EnvFilter::try_from_env("RUST_LOG")
            .unwrap_or(EnvFilter::new(LevelFilter::INFO.to_string())),
    );
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
