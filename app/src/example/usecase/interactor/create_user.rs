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
        let user = connection_factory
            .begin_transaction(|conn| async move {
                let id = Uuid::new_v4();
                let entity = UserEntity::new(id, user_name.to_string());
                let user_repository = UserRepositoryImpl {};
                let user = user_repository.create(entity, conn).await?;

                Ok(user)
            })
            .await?;
        Ok(user)
    }
}
