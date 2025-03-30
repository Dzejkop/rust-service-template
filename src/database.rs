#![allow(async_fn_in_trait)]

use std::ops::{Deref, DerefMut};

use sqlx::PgPool;

use crate::config::DbConfig;

pub mod something;

pub struct Db {
    pool: PgPool,
}

impl Db {
    pub async fn new(config: &DbConfig) -> sqlx::Result<Self> {
        let pool = PgPool::connect(&config.connection_string).await?;

        Ok(Self { pool })
    }

    /// Applies migrations
    ///
    /// Useful for testing
    pub async fn apply_migrations(&self) -> sqlx::Result<()> {
        sqlx::migrate!("./migrations").run(&self.pool).await?;

        Ok(())
    }
}

impl Deref for Db {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}

impl DerefMut for Db {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.pool
    }
}

#[cfg(test)]
mod tests {
    use testcontainers::{ContainerAsync, runners::AsyncRunner};
    use testcontainers_modules::postgres;

    use super::*;

    /// This funciton sets up a one-time database in a postgres container and applies all the
    /// migrations
    ///
    /// Make sure to keep the container object around until the end of the test
    pub async fn setup() -> eyre::Result<(Db, ContainerAsync<postgres::Postgres>)> {
        let container = postgres::Postgres::default().start().await?;
        let db_host = container.get_host().await?;
        let db_port = container.get_host_port_ipv4(5432).await?;

        // Get the connection string to the test database
        let db_url = format!("postgres://postgres:postgres@{db_host}:{db_port}/postgres",);

        // Create a database connection pool
        let db = Db::new(&DbConfig {
            connection_string: db_url.clone(),
        })
        .await?;

        db.apply_migrations().await?;

        Ok((db, container))
    }
}
