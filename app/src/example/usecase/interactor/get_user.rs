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

pub struct GetUserInteractor {}

impl GetUserInteractor {
    pub async fn execute(id: i32, db: &impl ConnectionFactory) -> Result<UserEntity, APIError> {
        let user = db
            .acquire(|pool| async move {
                let user_repository = UserRepositoryImpl {};
                let user = user_repository.find_by_id(id, &pool).await?;
                Ok(user)
            })
            .await?;
        Ok(user)
    }
}
