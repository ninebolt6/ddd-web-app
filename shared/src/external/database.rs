use std::time::Duration;

use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, PgPool, Postgres, Transaction};

async fn connect() -> Result<PgPool, Error> {
    PgPoolOptions::new()
        .min_connections(1)
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(8))
        .connect("postgresql://root:password@localhost:3306/database")
        .await
}

#[async_trait]
pub trait ConnectionFactory: Send + Sync + Clone {
    async fn acquire<F, T>(block: F) -> Result<T, Error>
    where
        F: Send + Fn(&PgPool) -> Result<T, Error>;

    async fn begin_transaction<F, T>(block: F) -> Result<T, Error>
    where
        F: Send + Fn(&Transaction<'_, Postgres>) -> Result<T, Error>,
        T: Send;
}

pub struct ConnectionFactoryImpl;

impl Clone for ConnectionFactoryImpl {
    fn clone(&self) -> Self {
        Self {}
    }
}

#[async_trait]
impl ConnectionFactory for ConnectionFactoryImpl {
    async fn acquire<F, T>(block: F) -> Result<T, Error>
    where
        F: Send + Fn(&PgPool) -> Result<T, Error>,
    {
        let pool = connect().await?;
        let result = block(&pool)?;

        Ok(result)
    }

    async fn begin_transaction<F, T>(block: F) -> Result<T, Error>
    where
        F: Send + Fn(&Transaction<'_, Postgres>) -> Result<T, Error>,
        T: Send,
    {
        let pool = connect().await?;
        let transaction = pool.begin().await?;

        let result = block(&transaction)?; // NOTE: ここでエラーが起きたらスコープを抜けた時にロールバックされる
        transaction.commit().await?;

        Ok(result)
    }
}
