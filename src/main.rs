#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]

use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use anyhow::Result;
use axum::{
    error_handling::HandleErrorLayer,
    extract::Extension,
    routing::{get, post},
    Router, Server,
};
use tokio_shutdown::Shutdown;
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing::{info, warn, Level};
use tracing_subscriber::{filter::Targets, prelude::*};

mod db;
mod dirs;
mod extractors;
mod handlers;
mod report;
mod retrace;
mod settings;
mod templates;

const ADDRESS: Ipv4Addr = if cfg!(debug_assertions) {
    Ipv4Addr::LOCALHOST
} else {
    Ipv4Addr::UNSPECIFIED
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            Targets::new()
                .with_target(env!("CARGO_CRATE_NAME"), Level::TRACE)
                .with_target("tower_http", Level::TRACE)
                .with_default(Level::INFO),
        )
        .init();

    let settings = settings::load()?;
    let settings = Arc::new(settings.auth);

    let pool = crate::db::create_pool()?;
    crate::db::run_migrations(&pool)?;

    let app = Router::new()
        .route("/", get(|| async { handlers::index() }))
        .nest(
            "/users",
            Router::new()
                .route(
                    "/create",
                    get(handlers::users::create).post(handlers::users::create_post),
                )
                .route("/", get(handlers::users::list)),
        )
        .nest(
            "/apps",
            Router::new()
                .route("/:id", get(handlers::versions_list))
                .route(
                    "/create",
                    get(handlers::apps::create).post(|| async { handlers::apps::create_post() }),
                )
                .route("/", get(handlers::apps::list)),
        )
        .route("/report", post(handlers::report_save))
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handlers::error::timeout))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(Extension(pool))
                .layer(Extension(settings))
                .into_inner(),
        );

    let addr = SocketAddr::from((ADDRESS, 8080));
    let shutdown = Shutdown::new()?;

    let server = Server::try_bind(&addr)?
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown.handle());

    info!("listening on http://{}", addr);

    server.await?;

    Ok(())
}
