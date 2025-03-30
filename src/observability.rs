use std::borrow::Cow;

use fastrace::collector::Reporter;
use fastrace::prelude::*;
use opentelemetry_otlp::WithExportConfig;
use crate::config::observability::ObservabilityConfig;

pub struct AggregateReporter {
    jaeger: Option<fastrace_jaeger::JaegerReporter>,
    datadog: Option<fastrace_datadog::DatadogReporter>,
    opentelemetry: Option<fastrace_opentelemetry::OpenTelemetryReporter>,
}

impl AggregateReporter {
    pub fn create(config: ObservabilityConfig) -> AggregateReporter {
        let jaeger = config.jaeger.map(|j| {
            fastrace_jaeger::JaegerReporter::new(j.agent_addr, j.service_name).unwrap()
        });
        let datadog = config.datadog.map(|d| {
            fastrace_datadog::DatadogReporter::new(
                d.agent_addr,
                d.tracer_mode,
                d.service_name,
                d.env,
            )
        });
        let opentelemetry = config.opentelemetry.map(|o| {
            fastrace_opentelemetry::OpenTelemetryReporter::new(
                opentelemetry_otlp::SpanExporter::builder()
                    .with_tonic()
                    .with_endpoint(o.endpoint)
                    .with_protocol(match o.protocol.to_lowercase().as_str() {
                        "grpc" => opentelemetry_otlp::Protocol::Grpc,
                        "http" => opentelemetry_otlp::Protocol::HttpProto,
                        _ => opentelemetry_otlp::Protocol::Grpc,
                    })
                    .with_timeout(opentelemetry_otlp::OTEL_EXPORTER_OTLP_TIMEOUT_DEFAULT)
                    .build()
                    .expect("initialize otlp exporter"),
                opentelemetry::trace::SpanKind::Server,
                Cow::Owned(
                    opentelemetry_sdk::Resource::builder()
                        .with_attributes([opentelemetry::KeyValue::new(
                            "service.name",
                            "custom_opentelemetry",
                        )])
                        .build(),
                ),
                opentelemetry::InstrumentationScope::builder("example-crate")
                    .with_version(env!("CARGO_PKG_VERSION"))
                    .build(),
            )
        });
        AggregateReporter {
            jaeger,
            datadog,
            opentelemetry,
        }
    }
}

impl Reporter for AggregateReporter {
    fn report(&mut self, spans: Vec<SpanRecord>) {
        if let Some(ref mut j) = self.jaeger {
            j.report(spans.clone());
        }
        if let Some(ref mut d) = self.datadog {
            d.report(spans.clone());
        }
        if let Some(ref mut o) = self.opentelemetry {
            o.report(spans);
        }
    }
}

