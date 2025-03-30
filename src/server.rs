use async_signal::{Signal, Signals};
use bon::builder;
use futures::StreamExt;
use poem::{Route, listener::TcpAcceptor};
use poem_openapi::{OpenApi, OpenApiService, param::Query, payload::PlainText};
use tokio::net::TcpListener;

pub struct App {}

#[OpenApi]
impl App {
    #[oai(path = "/echo", method = "get")]
    pub async fn echo(&self, text: Query<String>) -> PlainText<String> {
        PlainText(text.0)
    }
}

#[builder]
pub async fn serve(
    app: App,
    #[builder(default)] servers: Vec<String>,
    listener: TcpListener,
) -> eyre::Result<()> {
    let mut api_service =
        OpenApiService::new(app, env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    for server in servers {
        api_service = api_service.server(server);
    }

    let ui = api_service.swagger_ui();

    let app = Route::new().nest("/api", api_service).nest("/explore", ui);

    let acceptor = TcpAcceptor::from_tokio(listener)?;
    poem::Server::new_with_acceptor(acceptor)
        .run_with_graceful_shutdown(
            app,
            async {
                let mut signals = Signals::new([Signal::Term, Signal::Int, Signal::Quit])
                    .expect("Failed to construct signal handler");

                signals.next().await;
            },
            None,
        )
        .await?;

    Ok(())
}
