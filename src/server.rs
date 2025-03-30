use async_signal::{Signal, Signals};
use bon::builder;
use futures::StreamExt;
use poem::{Route, listener::TcpListener};
use poem_openapi::{OpenApi, OpenApiService, param::Query, payload::PlainText};

pub struct App {}

#[OpenApi]
impl App {
    #[oai(path = "/echo", method = "get")]
    pub async fn echo(&self, text: Query<String>) -> PlainText<String> {
        PlainText(text.0)
    }
}

#[builder]
pub async fn serve(app: App, servers: Vec<String>) -> eyre::Result<()> {
    let mut api_service = OpenApiService::new(app, "App", "1.0");
    let ui = api_service.swagger_ui();

    for server in servers {
        api_service = api_service.server(server);
    }

    let app = Route::new().nest("/api", api_service).nest("/explore", ui);

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
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
