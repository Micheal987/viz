//! [OpenTelemetry(OTEL) Prometheus Exporter][OTEL].
//!
//! [OTEL]: https://docs.rs/opentelemetry-prometheus

use http_body_util::Full;
use opentelemetry::otel_error;
use prometheus::{Encoder, TextEncoder};

use viz_core::{
    header::{HeaderValue, CONTENT_TYPE},
    Handler, IntoResponse, Request, Response, Result, StatusCode,
};

#[doc(inline)]
pub use opentelemetry_prometheus::ExporterBuilder;
#[doc(inline)]
pub use prometheus::Registry;

/// The [`Registry`] wrapper.
#[derive(Clone, Debug)]
pub struct Prometheus {
    registry: Registry,
}

impl Prometheus {
    /// Creates a new [`Prometheus`].
    #[must_use]
    pub const fn new(registry: Registry) -> Self {
        Self { registry }
    }
}

#[viz_core::async_trait]
impl Handler<Request> for Prometheus {
    type Output = Result<Response>;

    async fn call(&self, _: Request) -> Self::Output {
        let metric_families = self.registry.gather();
        let encoder = TextEncoder::new();
        let mut body = Vec::new();

        if let Err(err) = encoder.encode(&metric_families, &mut body) {
            let error = StatusCode::INTERNAL_SERVER_ERROR;
            let text = err.to_string();
            otel_error!(name: "prometheus_encode_failure", error_code = error.as_u16(), error = text.clone());
            Err((error, text).into_error())?;
        }

        let mut res = Response::new(Full::from(body).into());

        res.headers_mut().append(
            CONTENT_TYPE,
            HeaderValue::from_str(encoder.format_type())
                .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_error())?,
        );

        Ok(res)
    }
}
