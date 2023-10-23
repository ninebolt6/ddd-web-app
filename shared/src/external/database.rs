use std::future::Future;
use std::time::Duration;

use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error as DbError, PgPool, Postgres, Transaction};

use crate::common::error::APIError;

pub async fn connect_db() -> Result<PgPool, DbError> {
    PgPoolOptions::new()
        .min_connections(1)
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(8))
        .connect("postgresql://app:appUserPassword@localhost:5432/postgres")
        .await
}

#[async_trait]
pub trait ConnectionFactory {
    async fn acquire<F, T, Fut>(&self, block: F) -> Result<T, APIError>
    where
        F: FnOnce(PgPool) -> Fut + Send + Sync,
        Fut: Future<Output = Result<T, APIError>> + Send;

    async fn begin_transaction<'a, F, T, Fut>(&self, block: F) -> Result<T, APIError>
    where
        F: FnOnce(Transaction<'a, Postgres>) -> Fut + Send + Sync,
        Fut: Future<Output = Result<T, APIError>> + Send,
        T: Send;
}

pub struct ConnectionFactoryImpl {
    pool: PgPool,
}

impl ConnectionFactoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ConnectionFactory for ConnectionFactoryImpl {
    async fn acquire<F, T, Fut>(&self, block: F) -> Result<T, APIError>
    where
        F: FnOnce(PgPool) -> Fut + Send + Sync,
        Fut: Future<Output = Result<T, APIError>> + Send,
    {
        let result = block(self.pool.clone())
            .await
            .map_err(|e| APIError::InfrastructureError(e.to_string()))?;

        Ok(result)
    }

    async fn begin_transaction<'a, F, T, Fut>(&self, block: F) -> Result<T, APIError>
    where
        F: FnOnce(Transaction<'a, Postgres>) -> Fut + Send + Sync,
        Fut: Future<Output = Result<T, APIError>> + Send,
        T: Send,
    {
        let transaction = self
            .pool
            .begin()
            .await
            .map_err(|e| APIError::InfrastructureError(e.to_string()))?;

        let result = block(transaction).await?; // NOTE: ここでエラーが起きたらスコープを抜けた時にロールバックされる

        // transaction
        //     .commit()
        //     .await
        //     .map_err(|e| APIError::InfrastructureError(e.to_string()))?;

        Ok(result)
    }
}
