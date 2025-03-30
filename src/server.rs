use crate::error::Error;
use async_signal::{Signal, Signals};
use bon::builder;
use fastrace_poem::FastraceMiddleware;
use futures::StreamExt;
use poem::{EndpointExt, Route, listener::TcpAcceptor};
use poem_openapi::{
    OpenApi, OpenApiService,
    param::Path,
    payload::{Json, PlainText},
};
use tokio::net::TcpListener;

use crate::{
    config::ServiceConfig,
    database::{Db, something::Something},
};

pub struct App {
    db: Db,
}

impl App {
    pub async fn new(config: &ServiceConfig) -> eyre::Result<Self> {
        let db = crate::database::Db::new(&config.db).await?;

        Ok(Self { db })
    }
}

#[OpenApi]
impl App {
    /// Echoes the text back
    #[oai(path = "/echo", method = "post")]
    pub async fn echo(&self, text: PlainText<String>) -> PlainText<String> {
        PlainText(text.0)
    }

    /// Healthcheck method
    ///
    /// for now it always returns 200
    #[oai(path = "/health", method = "get")]
    pub async fn health(&self) {}

    /// Saves a value to the db
    #[oai(path = "/something/:something", method = "post")]
    pub async fn create_something(&self, something: Path<String>) -> Result<(), Error> {
        self.db.insert_something(something.0).await?;

        Ok(())
    }

    #[oai(path = "/somethings", method = "get")]
    pub async fn get_somethings(&self) -> Result<Json<Vec<String>>, Error> {
        let all = self.db.fetch_all().await?;

        Ok(Json(all))
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
    let spec = api_service.spec();

    let app = Route::new()
        .nest("/", api_service)
        .nest("/swagger", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(FastraceMiddleware);

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
