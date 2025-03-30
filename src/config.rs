use serde::{Deserialize, Serialize};

pub mod observability;

pub const CONFIG_PREFIX: &str = "APP";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub db: DbConfig,
    #[serde(default)]
    pub observability: observability::ObservabilityConfig,
}

impl ServiceConfig {
    pub fn load() -> eyre::Result<Self> {
        let mut config_builder = config::Config::builder();

        config_builder = config_builder.add_source(
            config::Environment::with_prefix(CONFIG_PREFIX)
                .separator("__")
                .try_parsing(true),
        );

        let settings = config_builder.build()?;

        let config = serde_path_to_error::deserialize(settings)?;

        Ok(config)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DbConfig {
    /// The db connection string
    /// i.e. postgresql://user:password@localhost:5432/dbname
    pub connection_string: String,

    /// Whether to create the database if it does not exist
    #[serde(default = "default::bool_true")]
    pub create: bool,

    /// Whether to run migrations
    #[serde(default = "default::bool_true")]
    pub migrate: bool,
}

mod default {
    pub fn bool_true() -> bool {
        true
    }
}
