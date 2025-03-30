use fastrace::collector::Config;
use logforth::{append, filter::EnvFilter};
use observability::AggregateReporter;
use tokio::net::TcpListener;

pub mod config;
pub mod database;
pub mod error;
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

    let app = server::App::new(&config).await?;

    let listener = TcpListener::bind(config.server.host).await?;

    log::info!("Server listening on {}", listener.local_addr()?);
    let mut servers = vec![];
    if let Some(server) = config.server.server {
        log::info!("Will server explorer at {server}");
        servers.push(server);
    }
    testing_traces().await;

    server::serve()
        .app(app)
        .servers(servers)
        .listener(listener)
        .call()
        .await?;

    fastrace::flush();

    Ok(())
}

#[tracing::instrument]
async fn testing_traces() {
    tracing::info!("Tracing macros work!");
    log::info!("Log also works!");
}
