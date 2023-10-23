use async_trait::async_trait;
use sqlx::{Executor, FromRow, PgConnection};
use uuid::Uuid;

use crate::{common::error::APIError, example::domain::entity::user::UserEntity};

#[async_trait]
pub trait UserRepository {
    async fn find_by_id<'a, T>(&self, id: Uuid, conn: T) -> Result<Option<UserEntity>, APIError>
    where
        T: Executor<'a, Database = sqlx::Postgres>;
    async fn create(&self, entity: UserEntity, conn: &mut PgConnection) -> Result<(), APIError>;
}

pub struct UserRepositoryImpl {}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id<'a, T>(&self, id: Uuid, conn: T) -> Result<Option<UserEntity>, APIError>
    where
        T: Executor<'a, Database = sqlx::Postgres>,
    {
        #[derive(FromRow)]
        struct UserRow {
            id: Uuid,
            name: String,
        }

        let user_row = sqlx::query_as::<_, UserRow>("SELECT id, name FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(conn)
            .await
            .map_err(|e| APIError::InfrastructureError(e.to_string()))?;

        let user = user_row.map(|row| UserEntity::new(row.id, row.name));
        Ok(user)
    }

    async fn create(&self, entity: UserEntity, conn: &mut PgConnection) -> Result<(), APIError> {
        sqlx::query("INSERT INTO users (id, name) VALUES ($1, $2)")
            .bind(entity.id)
            .bind(entity.name)
            .execute(&mut *conn)
            .await
            .map_err(|e| APIError::InfrastructureError(e.to_string()))?;
        Ok(())
    }
}
