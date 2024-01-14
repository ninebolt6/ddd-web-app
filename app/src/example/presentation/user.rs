use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared::{
    common::result::ResponseResult,
    example::infrastructure::database::repository::user_repository::UserRepositoryImpl,
    external::database::{ConnectionFactory, ConnectionFactoryImpl},
};

use crate::example::usecase::interactor::{
    create_user::CreateUserInteractor, get_user::GetUserInteractor,
};

pub fn example_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users").route(web::post().to(create_user::<ConnectionFactoryImpl>)),
    )
    .service(web::resource("/users/{id}").route(web::get().to(get_user::<ConnectionFactoryImpl>)));
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct GetUserResponse {
    pub id: Uuid,
    pub name: String,
}

async fn get_user<CF>(path: web::Path<Uuid>, connection_factory: web::Data<CF>) -> ResponseResult
where
    CF: ConnectionFactory,
{
    let id = path.into_inner();
    let user_repository = UserRepositoryImpl {};

    let output =
        GetUserInteractor::execute(id, user_repository, connection_factory.into_inner()).await?;

    Ok(HttpResponse::Ok().json(GetUserResponse {
        id: output.id,
        name: output.name,
    }))
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    pub user_name: String,
}

async fn create_user<CF>(
    body: web::Json<CreateUserRequest>,
    connection_factory: web::Data<CF>,
) -> ResponseResult
where
    CF: ConnectionFactory,
{
    let user_name = &body.user_name;
    let user_repository = UserRepositoryImpl {};

    CreateUserInteractor::execute(
        user_name.to_string(),
        user_repository,
        connection_factory.into_inner(),
    )
    .await?;

    Ok(HttpResponse::Created().finish())
}
