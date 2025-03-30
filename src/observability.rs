use std::borrow::Cow;

use fastrace::collector::Reporter;
use fastrace::prelude::*;
use opentelemetry_otlp::WithExportConfig;

pub struct AggregateReporter {
    jaeger: fastrace_jaeger::JaegerReporter,
    datadog: fastrace_datadog::DatadogReporter,
    opentelemetry: fastrace_opentelemetry::OpenTelemetryReporter,
}

impl AggregateReporter {
    pub fn create() -> AggregateReporter {
        AggregateReporter {
            jaeger: fastrace_jaeger::JaegerReporter::new(
                "127.0.0.1:6831".parse().unwrap(),
                "asynchronous",
            )
            .unwrap(),
            datadog: fastrace_datadog::DatadogReporter::new(
                "127.0.0.1:8126".parse().unwrap(),
                "asynchronous",
                "db",
                "select",
            ),
            opentelemetry: fastrace_opentelemetry::OpenTelemetryReporter::new(
                opentelemetry_otlp::SpanExporter::builder()
                    .with_tonic()
                    .with_endpoint("http://127.0.0.1:4317".to_string())
                    .with_protocol(opentelemetry_otlp::Protocol::Grpc)
                    .with_timeout(opentelemetry_otlp::OTEL_EXPORTER_OTLP_TIMEOUT_DEFAULT)
                    .build()
                    .expect("initialize oltp exporter"),
                opentelemetry::trace::SpanKind::Server,
                Cow::Owned(
                    opentelemetry_sdk::Resource::builder()
                        .with_attributes([opentelemetry::KeyValue::new(
                            "service.name",
                            "asynchronous(opentelemetry)",
                        )])
                        .build(),
                ),
                opentelemetry::InstrumentationScope::builder("example-crate")
                    .with_version(env!("CARGO_PKG_VERSION"))
                    .build(),
            ),
        }
    }
}

impl Reporter for AggregateReporter {
    fn report(&mut self, spans: Vec<SpanRecord>) {
        self.jaeger.report(spans.clone());
        self.datadog.report(spans.clone());
        self.opentelemetry.report(spans);
    }
}

