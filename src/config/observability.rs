use std::net::SocketAddr;

pub struct ObservabilityConfig {}

pub struct JaegerConfig {
    pub agent_addr: SocketAddr,
    pub service_name: String,
}
