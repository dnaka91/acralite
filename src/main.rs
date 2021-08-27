#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]

use std::{env, net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Result;
use axum::{prelude::*, AddExtensionLayer};
use hyper::Server;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{info, warn};

mod db;
mod extractors;
mod handlers;
mod report;
mod responses;
mod retrace;
mod settings;
mod templates;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    env::set_var(
        "RUST_LOG",
        "info,acralite=debug,axum=debug,tower_http=debug",
    );
    tracing_subscriber::fmt::init();

    let settings = settings::load()?;
    let settings = Arc::new(settings.auth);

    let pool = crate::db::create_pool()?;
    crate::db::run_migrations(&pool)?;

    let app = route("/", get(|| async { handlers::index() }))
        .nest(
            "/users",
            route(
                "/create",
                get(handlers::users::create).post(handlers::users::create_post),
            )
            .route("/", get(handlers::users::list)),
        )
        .nest(
            "/apps",
            route("/:id", get(handlers::versions_list))
                .route(
                    "/create",
                    get(handlers::apps::create).post(|| async { handlers::apps::create_post() }),
                )
                .route("/", get(handlers::apps::list)),
        )
        .route("/report", post(handlers::report_save))
        .layer(
            ServiceBuilder::new()
                .timeout(Duration::from_secs(10))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|_: &Request<Body>| tracing::debug_span!("api"))
                        .on_request(())
                        .on_response(DefaultOnResponse::new().latency_unit(LatencyUnit::Micros)),
                )
                .layer(CompressionLayer::new())
                .layer(AddExtensionLayer::new(pool))
                .layer(AddExtensionLayer::new(settings))
                .into_inner(),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));

    let server = Server::try_bind(&addr)?
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            signal::ctrl_c().await.ok();
        });

    info!("listening on {}", addr);

    server.await?;

    Ok(())
}
