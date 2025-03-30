use std::borrow::Cow;

use crate::config::observability::ObservabilityConfig;
use fastrace::collector::Reporter;
use fastrace::prelude::*;
use opentelemetry_otlp::WithExportConfig;

pub struct AggregateReporter {
    jaeger: Option<fastrace_jaeger::JaegerReporter>,
    datadog: Option<fastrace_datadog::DatadogReporter>,
    opentelemetry: Option<fastrace_opentelemetry::OpenTelemetryReporter>,
}

impl AggregateReporter {
    pub fn create(config: &ObservabilityConfig) -> AggregateReporter {
        let jaeger = config.jaeger.as_ref().map(|j| {
            fastrace_jaeger::JaegerReporter::new(j.agent_addr, j.service_name.clone()).unwrap()
        });

        let datadog = config.datadog.as_ref().map(|d| {
            fastrace_datadog::DatadogReporter::new(
                d.agent_addr,
                d.tracer_mode.clone(),
                d.service_name.clone(),
                d.env.clone(),
            )
        });

        let opentelemetry = if let Some(o) = &config.opentelemetry {
            let span_exporter = opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .with_endpoint(o.endpoint.clone())
                .with_protocol(o.protocol)
                .with_timeout(opentelemetry_otlp::OTEL_EXPORTER_OTLP_TIMEOUT_DEFAULT)
                .build()
                .expect("initialize otlp exporter");

            Some(fastrace_opentelemetry::OpenTelemetryReporter::new(
                span_exporter,
                opentelemetry::trace::SpanKind::Server,
                Cow::Owned(
                    opentelemetry_sdk::Resource::builder()
                        .with_attributes([opentelemetry::KeyValue::new(
                            "service.name",
                            o.service_name.clone(),
                        )])
                        .build(),
                ),
                opentelemetry::InstrumentationScope::builder(o.scope.clone())
                    .with_version(env!("CARGO_PKG_VERSION"))
                    .build(),
            ))
        } else {
            None
        };

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
