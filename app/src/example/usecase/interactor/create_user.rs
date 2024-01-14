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
        let user_repository = UserRepositoryImpl {};

        connection_factory
            .begin_transaction(|tx| {
                Box::pin(async move {
                    let entity = UserEntity::new(user_name);
                    user_repository.create(entity, tx).await
                })
            })
            .await
    }
}
