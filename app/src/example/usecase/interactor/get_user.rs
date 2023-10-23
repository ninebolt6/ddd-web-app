use actix_web::web::Data;
use shared::{
    common::error::APIError,
    example::{
        domain::entity::user::UserEntity,
        infrastructure::database::repository::user_repository::{
            UserRepository, UserRepositoryImpl,
        },
    },
    external::database::ConnectionFactory,
};
use uuid::Uuid;

pub struct GetUserInteractor {}

impl GetUserInteractor {
    pub async fn execute<CF>(id: Uuid, connection_factory: Data<CF>) -> Result<UserEntity, APIError>
    where
        CF: ConnectionFactory,
    {
        let user = connection_factory
            .acquire(|pool| async move {
                let mut conn = pool.acquire().await.map_err(|e| {
                    APIError::InfrastructureError(format!("Failed to acquire connection: {}", e))
                })?;

                let user_repository = UserRepositoryImpl {};
                user_repository.find_by_id(id, &mut conn).await
            })
            .await?
            .ok_or(APIError::NotFound("Not Found".to_string()))?;

        Ok(user)
    }
}
