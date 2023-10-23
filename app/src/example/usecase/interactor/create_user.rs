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

pub struct CreateUserInteractor {}

impl CreateUserInteractor {
    pub async fn execute<CF>(user_name: &str, connection_factory: Data<CF>) -> Result<(), APIError>
    where
        CF: ConnectionFactory,
    {
        connection_factory
            .acquire(|pool| async move {
                let mut conn = pool.acquire().await.map_err(|e| {
                    APIError::InfrastructureError(format!("Failed to acquire connection: {}", e))
                })?;

                let id = Uuid::new_v4();
                let entity = UserEntity::new(id, user_name.to_string());
                let user_repository = UserRepositoryImpl {};

                user_repository.create(entity, &mut conn).await?;

                Ok(())
            })
            .await?;
        Ok(())
    }
}
