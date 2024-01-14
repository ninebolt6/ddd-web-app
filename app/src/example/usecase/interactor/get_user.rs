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

pub struct GetUserInteractorOutput {
    pub id: Uuid,
    pub name: String,
}

impl From<UserEntity> for GetUserInteractorOutput {
    fn from(value: UserEntity) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

impl GetUserInteractor {
    pub async fn execute<CF>(
        id: Uuid,
        connection_factory: Data<CF>,
    ) -> Result<GetUserInteractorOutput, APIError>
    where
        CF: ConnectionFactory,
    {
        let user = connection_factory
            .acquire(move |pool| async move {
                let mut conn = pool.acquire().await.map_err(|e| {
                    APIError::InfrastructureError(format!("Failed to acquire connection: {}", e))
                })?;

                let user_repository = UserRepositoryImpl {};
                user_repository.find_by_id(id, &mut conn).await
            })
            .await?
            .ok_or(APIError::NotFound("Not Found".to_string()))?;

        Ok(GetUserInteractorOutput::from(user))
    }
}
