use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct ObservabilityConfig {
    pub jaeger: Option<JaegerConfig>,
    pub datadog: Option<DatadogConfig>,
    pub opentelemetry: Option<OpenTelemetryConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct JaegerConfig {
    pub agent_addr: SocketAddr,
    pub service_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct DatadogConfig {
    pub agent_addr: SocketAddr,
    pub tracer_mode: String,
    pub service_name: String,
    pub env: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct OpenTelemetryConfig {
    pub endpoint: String,
    pub protocol: String,
}
