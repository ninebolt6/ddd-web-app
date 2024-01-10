use actix_web::{
    http::StatusCode,
    test::{self, read_body},
    web::{self, Data},
    App,
};
use app::example::presentation::user::{CreateUserRequest, GetUserResponse};
use app::route::{auth_routes, public_routes};
use shared::external::database::{connect_test_db, ConnectionFactoryImpl};
use sqlx::FromRow;
use uuid::Uuid;

#[actix_web::test]
async fn test_get_user() {
    // arrange
    let pool = connect_test_db().await.unwrap();
    let user_id = Uuid::new_v4();

    sqlx::query("INSERT INTO users (id, name) VALUES ($1, $2)")
        .bind(user_id)
        .bind("ユーザ名")
        .execute(&pool)
        .await
        .unwrap();

    let app = test::init_service(
        App::new()
            .service(
                web::scope("/api").configure(public_routes).service(
                    web::scope("")
                        // .wrap(auth_middleware)
                        .configure(auth_routes),
                ),
            )
            .app_data(Data::new(ConnectionFactoryImpl::new(pool))),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(format!("/api/users/{user_id}").as_str())
        .to_request();

    // act
    let resp = test::call_service(&app, req).await;

    // assert
    assert_eq!(resp.status(), StatusCode::OK);

    let resp_body = read_body(resp).await;
    let expected = GetUserResponse {
        id: user_id,
        name: "ユーザ名".to_string(),
    };

    assert_eq!(
        std::str::from_utf8(&resp_body).unwrap(),
        serde_json::to_string(&expected).unwrap(),
    );
}

#[actix_web::test]
async fn test_create_user() {
    // arrange
    let pool = connect_test_db().await.unwrap();

    let app = test::init_service(
        App::new()
            .service(
                web::scope("/api").configure(public_routes).service(
                    web::scope("")
                        // .wrap(auth_middleware)
                        .configure(auth_routes),
                ),
            )
            .app_data(Data::new(ConnectionFactoryImpl::new(pool.clone()))),
    )
    .await;

    let req_body = CreateUserRequest {
        // random username
        user_name: Uuid::new_v4().to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(&req_body)
        .to_request();

    // act
    let resp = test::call_service(&app, req).await;

    // assert
    assert_eq!(resp.status(), StatusCode::CREATED);

    #[derive(FromRow)]
    struct UserRow {
        id: Uuid,
        name: String,
    }
    let result = sqlx::query_as::<_, UserRow>("SELECT id, name FROM users WHERE name = $1")
        .bind(req_body.user_name)
        .fetch_one(&pool)
        .await;

    assert!(result.is_ok())
}
