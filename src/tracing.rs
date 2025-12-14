use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    Layer as _,
    fmt::{self, format::FmtSpan, writer::MakeWriterExt as _},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

use crate::config::APP_CONFIG;

pub(crate) fn init_tracing() -> WorkerGuard {
    // env filter
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);
    let console_layer = fmt::Layer::new()
        .with_writer(stdout)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .pretty()
        .with_filter(env_filter.clone());

    // file appender layer for tracing-subscriber
    let file_appender = tracing_appender::rolling::daily(
        &APP_CONFIG.log.dir.clone(),
        "server.log",
    );
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_ansi(false)
        .with_filter(env_filter.clone());

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();

    guard
}
