#![allow(clippy::unused_async)]

use opentelemetry::global;
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    runtime::TokioCurrentThread,
    {propagation::TraceContextPropagator, trace::TracerProvider},
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use viz::{middleware::otel, serve, Error, Request, Result, Router};

fn init_tracer_provider() -> TracerProvider {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let exporter = SpanExporter::builder()
        .with_http()
        .with_protocol(opentelemetry_otlp::Protocol::HttpBinary)
        .build()
        .unwrap();

    TracerProvider::builder()
        .with_batch_exporter(exporter, TokioCurrentThread)
        .build()
}

async fn index(_: Request) -> Result<&'static str> {
    Ok("Hello, World!")
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    println!("listening on http://{addr}");

    let tracer_provider = init_tracer_provider();

    let app = Router::new()
        .get("/", index)
        .get("/:username", index)
        .with(otel::tracing::Config::new(tracer_provider.clone(), None));

    if let Err(e) = serve(listener, app).await {
        println!("{e}");
    }

    // Ensure all spans have been reported
    global::shutdown_tracer_provider();
    tracer_provider.shutdown().map_err(Error::boxed)?;

    Ok(())
}
