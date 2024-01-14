use std::sync::Arc;

use shared::{
    common::error::APIError,
    example::{
        domain::entity::user::UserEntity,
        infrastructure::database::repository::user_repository::UserRepository,
    },
    external::database::ConnectionFactory,
};

pub struct CreateUserInteractor {}

impl CreateUserInteractor {
    pub async fn execute<CF>(
        user_name: String,
        user_repository: impl UserRepository + Send + 'static,
        connection_factory: Arc<CF>,
    ) -> Result<(), APIError>
    where
        CF: ConnectionFactory,
    {
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
