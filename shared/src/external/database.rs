use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error as DbError, PgConnection, PgPool};

use crate::common::error::APIError;

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

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

    async fn begin_transaction<'b, F, T>(&self, block: F) -> Result<T, APIError>
    where
        F: for<'e> FnOnce(&'e mut PgConnection) -> BoxFuture<'e, Result<T, APIError>> + Send,
        T: Send;
}

pub trait DbPool {}
impl DbPool for PgPool {}

pub struct ConnectionFactoryImpl<T: DbPool = PgPool> {
    pool: T,
}

impl ConnectionFactoryImpl<PgPool> {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ConnectionFactory for ConnectionFactoryImpl<PgPool> {
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

    async fn begin_transaction<'b, F, T>(&self, block: F) -> Result<T, APIError>
    where
        F: for<'e> FnOnce(&'e mut PgConnection) -> BoxFuture<'e, Result<T, APIError>> + Send,
        T: Send,
    {
        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|e| APIError::InfrastructureError(e.to_string()))?;

        let result = block(&mut transaction).await?; // NOTE: ここでエラーが起きたらスコープを抜けた時にロールバックされる

        transaction
            .commit()
            .await
            .map_err(|e| APIError::InfrastructureError(e.to_string()))?;

        Ok(result)
    }
}
