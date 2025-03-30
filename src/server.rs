use async_signal::{Signal, Signals};
use bon::builder;
use futures::StreamExt;
use poem::{Route, listener::TcpListener};
use poem_openapi::{OpenApi, OpenApiService, param::Query, payload::PlainText};
use std::net::SocketAddr;

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
    servers: Vec<String>,
    listener: Option<TcpListener>,
    socket_addr: Option<SocketAddr>,
) -> eyre::Result<()> {
    let mut api_service =
        OpenApiService::new(app, env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let ui = api_service.swagger_ui();

    for server in servers {
        api_service = api_service.server(server);
    }

    let app = Route::new().nest("/api", api_service).nest("/explore", ui);

    let listener = if let Some(listener) = listener {
        listener
    } else {
        let addr =
            socket_addr.unwrap_or_else(|| "0.0.0.0:3000".parse().expect("Invalid socket address"));
        TcpListener::bind(addr.to_string().as_str())
    };
    poem::Server::new(listener)
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
