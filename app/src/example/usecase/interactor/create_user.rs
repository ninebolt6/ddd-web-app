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

pub struct CreateUserInteractor {}

impl CreateUserInteractor {
    pub async fn execute<CF>(
        user_name: String,
        connection_factory: Data<CF>,
    ) -> Result<(), APIError>
    where
        CF: ConnectionFactory,
    {
        connection_factory
            .begin_transaction(|tx| {
                let user_repository = UserRepositoryImpl {};
                let entity = UserEntity::new(user_name.to_owned());

                Box::pin(async move { user_repository.create(entity, tx).await })
            })
            .await
    }
}
