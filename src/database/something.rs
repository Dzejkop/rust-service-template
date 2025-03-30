#![allow(async_fn_in_trait)]

use sqlx::{Acquire, Postgres};

/// Inspired by Elixir's Phoenix's "context" concept
///
/// This pattern allows us to treat any db "connection" (i.e. an actual connection, a pool, a
/// transaction) as an entry point for our high level db operations.
pub trait Something<'c>: Acquire<'c, Database = Postgres> + Sized {
    async fn insert_something(self, something: impl ToString) -> sqlx::Result<()> {
        let mut conn = self.acquire().await?;

        sqlx::query(
            r#"
            INSERT INTO something (value)
            VALUES ($1)
            "#,
        )
        .bind(something.to_string())
        .execute(&mut *conn)
        .await?;

        Ok(())
    }

    async fn fetch_all(self) -> sqlx::Result<Vec<String>> {
        let mut conn = self.acquire().await?;

        let values: Vec<(String,)> = sqlx::query_as(
            r#"
            SELECT value
            FROM something
            "#,
        )
        .fetch_all(&mut *conn)
        .await?;

        Ok(values.into_iter().map(|(value,)| value).collect())
    }
}

impl<'c, T> Something<'c> for T where T: Acquire<'c, Database = Postgres> + Send + Sync + Sized {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn add_and_fetch_stuff() -> eyre::Result<()> {
        let (db, _container) = crate::database::tests::setup().await?;

        db.insert_something("whatever").await?;
        db.insert_something("foo").await?;

        let mut all = db.fetch_all().await?;
        all.sort();

        assert_eq!(all, vec!["foo", "whatever"]);

        Ok(())
    }
}
