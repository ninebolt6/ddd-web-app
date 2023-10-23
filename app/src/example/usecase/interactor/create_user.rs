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
            .begin_transaction(|mut conn| async move {
                let id = Uuid::new_v4();
                let entity = UserEntity::new(id, user_name.to_string());
                let user_repository = UserRepositoryImpl {};

                user_repository.create(entity, conn.as_mut()).await
            })
            .await?;
        Ok(())
    }
}
