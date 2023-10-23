use std::future::Future;
use std::time::Duration;

use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error as DbError, PgPool, Postgres, Transaction};

use crate::common::error::APIError;

async fn connect() -> Result<PgPool, DbError> {
    PgPoolOptions::new()
        .min_connections(1)
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(8))
        .connect("postgresql://app:appUserPassword@localhost:5432/postgres")
        .await
}

#[async_trait]
pub trait ConnectionFactory: Clone {
    async fn acquire<F, T, Fut>(&self, block: F) -> Result<T, APIError>
    where
        F: Fn(PgPool) -> Fut + Send + Sync,
        Fut: Future<Output = Result<T, APIError>> + Send;

    async fn begin_transaction<F, T>(&self, block: F) -> Result<T, APIError>
    where
        F: Send + Fn(&Transaction<'_, Postgres>) -> Result<T, APIError>,
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
    async fn acquire<F, T, Fut>(&self, block: F) -> Result<T, APIError>
    where
        F: Fn(PgPool) -> Fut + Send + Sync,
        Fut: Future<Output = Result<T, APIError>> + Send,
    {
        let pool = connect()
            .await
            .map_err(|e| APIError::InfrastructureError(e.to_string()))?;

        let result = block(pool)
            .await
            .map_err(|e| APIError::InfrastructureError(e.to_string()))?;

        Ok(result)
    }

    async fn begin_transaction<F, T>(&self, block: F) -> Result<T, APIError>
    where
        F: Send + Fn(&Transaction<'_, Postgres>) -> Result<T, APIError>,
        T: Send,
    {
        let pool = connect()
            .await
            .map_err(|e| APIError::InfrastructureError(e.to_string()))?;
        let transaction = pool
            .begin()
            .await
            .map_err(|e| APIError::InfrastructureError(e.to_string()))?;

        let result = block(&transaction)?; // NOTE: ここでエラーが起きたらスコープを抜けた時にロールバックされる
        transaction
            .commit()
            .await
            .map_err(|e| APIError::InfrastructureError(e.to_string()))?;

        Ok(result)
    }
}
