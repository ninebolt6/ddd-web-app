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
    pub async fn execute(user_name: &str, db: &impl ConnectionFactory) -> Result<(), APIError> {
        let user = db
            .acquire(|pool| async move {
                let id = Uuid::new_v4();
                let entity = UserEntity::new(id, user_name.to_string());
                let user_repository = UserRepositoryImpl {};
                let user = user_repository.create(entity, &pool).await?;
                Ok(user)
            })
            .await?;
        Ok(user)
    }
}
