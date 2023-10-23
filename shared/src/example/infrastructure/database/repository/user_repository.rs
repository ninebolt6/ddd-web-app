use async_trait::async_trait;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{common::error::APIError, example::domain::entity::user::UserEntity};

#[async_trait]
pub trait UserRepository {
    async fn find_by_id(&self, id: Uuid, pool: &PgPool) -> Result<Option<UserEntity>, APIError>;
}

pub struct UserRepositoryImpl {}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, id: Uuid, pool: &PgPool) -> Result<Option<UserEntity>, APIError> {
        #[derive(FromRow)]
        struct UserRow {
            id: Uuid,
            name: String,
        }

        let user_row = sqlx::query_as::<_, UserRow>("SELECT id, name FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|e| APIError::InfrastructureError(e.to_string()))?;

        let user = user_row.map(|row| UserEntity::new(row.id, row.name));
        Ok(user)
    }
}
