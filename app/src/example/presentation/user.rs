use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared::{
    common::result::ResponseResult,
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

    let output = GetUserInteractor::execute(id, connection_factory).await?;

    Ok(HttpResponse::Ok().json(GetUserResponse {
        id: output.id,
        name: output.name,
    }))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    user_name: String,
}

async fn create_user<CF>(
    body: web::Json<CreateUserRequest>,
    connection_factory: web::Data<CF>,
) -> ResponseResult
where
    CF: ConnectionFactory,
{
    let user_name = &body.user_name;

    CreateUserInteractor::execute(user_name, connection_factory).await?;

    Ok(HttpResponse::Created().finish())
}

#[cfg(test)]
mod tests {
    use actix_web::{
        http::StatusCode,
        web::{self, Data},
    };

    use shared::external::database::connect_test_db;

    use super::*;

    #[actix_web::test]
    async fn test_create_user() {
        let body = web::Json(CreateUserRequest {
            user_name: "test name".to_string(),
        });
        let test_connection_factory = ConnectionFactoryImpl::new(connect_test_db().await.unwrap());

        let resp = create_user::<ConnectionFactoryImpl>(body, Data::new(test_connection_factory))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
    }
}
