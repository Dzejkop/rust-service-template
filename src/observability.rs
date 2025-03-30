use std::borrow::Cow;

use fastrace::collector::Reporter;
use fastrace::prelude::*;
use opentelemetry_otlp::WithExportConfig;
use crate::config::observability::ObservabilityConfig;

pub struct AggregateReporter {
    jaeger: fastrace_jaeger::JaegerReporter,
    datadog: fastrace_datadog::DatadogReporter,
    opentelemetry: fastrace_opentelemetry::OpenTelemetryReporter,
}

impl AggregateReporter {
    pub fn create(config: ObservabilityConfig) -> AggregateReporter {
        AggregateReporter {
            jaeger: match config.jaeger {
                Some(j) => fastrace_jaeger::JaegerReporter::new(
                    j.agent_addr,
                    j.service_name,
                ).unwrap(),
                None => fastrace_jaeger::JaegerReporter::new(
                    "127.0.0.1:6831".parse().unwrap(),
                    "default_jaeger".to_string(),
                ).unwrap(),
            },
            datadog: match config.datadog {
                Some(d) => fastrace_datadog::DatadogReporter::new(
                    d.agent_addr,
                    d.tracer_mode,
                    d.service_name,
                    d.env,
                ),
                None => fastrace_datadog::DatadogReporter::new(
                    "127.0.0.1:8126".parse().unwrap(),
                    "default_tracer".to_string(),
                    "default_datadog".to_string(),
                    "default_env".to_string(),
                ),
            },
            opentelemetry: match config.opentelemetry {
                Some(o) => fastrace_opentelemetry::OpenTelemetryReporter::new(
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
                ),
                None => fastrace_opentelemetry::OpenTelemetryReporter::new(
                    opentelemetry_otlp::SpanExporter::builder()
                        .with_tonic()
                        .with_endpoint("http://127.0.0.1:4317".to_string())
                        .with_protocol(opentelemetry_otlp::Protocol::Grpc)
                        .with_timeout(opentelemetry_otlp::OTEL_EXPORTER_OTLP_TIMEOUT_DEFAULT)
                        .build()
                        .expect("initialize otlp exporter"),
                    opentelemetry::trace::SpanKind::Server,
                    Cow::Owned(
                        opentelemetry_sdk::Resource::builder()
                            .with_attributes([opentelemetry::KeyValue::new(
                                "service.name",
                                "default_opentelemetry",
                            )])
                            .build(),
                    ),
                    opentelemetry::InstrumentationScope::builder("example-crate")
                        .with_version(env!("CARGO_PKG_VERSION"))
                        .build(),
                ),
            },
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

