use fastrace::collector::Config;
use logforth::{append, filter::EnvFilter};
use observability::AggregateReporter;

pub mod config;
pub mod database;
pub mod logging;
pub mod observability;
pub mod server;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();

    logforth::builder()
        .dispatch(|d| {
            d.filter(EnvFilter::from_default_env())
                .append(append::Stderr::default())
        })
        .dispatch(|d| d.append(append::FastraceEvent::default()))
        .apply();

    let config = config::ServiceConfig::load()?;

    fastrace::set_reporter(
        AggregateReporter::create(&config.observability),
        Config::default(),
    );

    log::info!("Starting!");

    testing_traces().await;

    fastrace::flush();

    Ok(())
}

#[tracing::instrument]
async fn testing_traces() {
    tracing::info!("Testing!");
}
