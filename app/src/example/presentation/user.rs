use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use shared::{
    common::{injector::Injector, result::ResponseResult},
    external::database::{ConnectionFactory, ConnectionFactoryImpl},
};
use uuid::Uuid;

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
struct GetUserResponse {
    id: Uuid,
    name: String,
}

async fn get_user<CF: ConnectionFactory>(
    path: web::Path<Uuid>,
    injector: web::Data<Injector<CF>>,
) -> ResponseResult {
    let id = path.into_inner();

    let output = GetUserInteractor::execute(id, injector.connection_factory()).await?;

    Ok(HttpResponse::Ok().json(GetUserResponse {
        id: output.id,
        name: output.name,
    }))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct CreateUserRequest {
    user_name: String,
}

async fn create_user<CF: ConnectionFactory>(
    body: web::Json<CreateUserRequest>,
    injector: web::Data<Injector<CF>>,
) -> ResponseResult {
    let user_name = &body.user_name;

    CreateUserInteractor::execute(user_name, injector.connection_factory()).await?;

    Ok(HttpResponse::Created().finish())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use actix_web::{
        body::to_bytes,
        http::StatusCode,
        web::{self, Data},
    };

    use super::*;

    #[actix_web::test]
    async fn test_get_user() {
        let id = Uuid::from_str("ca1c5eb2-a43a-4cef-80fc-f9abe1623785").unwrap();
        let path = web::Path::from(id);

        let resp = get_user::<ConnectionFactoryImpl>(
            path,
            Data::new(Injector::new(ConnectionFactoryImpl)),
        )
        .await
        .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        let resp_body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(
            serde_json::from_slice::<GetUserResponse>(&resp_body).unwrap(),
            GetUserResponse {
                id: id,
                name: "test name".to_string(),
            }
        );
    }

    #[actix_web::test]
    async fn test_create_user() {
        let body = web::Json(CreateUserRequest {
            user_name: "test name".to_string(),
        });

        let resp = create_user::<ConnectionFactoryImpl>(
            body,
            Data::new(Injector::new(ConnectionFactoryImpl)),
        )
        .await
        .unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
    }
}
