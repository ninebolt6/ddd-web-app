use actix_web::{
    http::StatusCode,
    test::{self, read_body},
    web::{self, Data},
    App,
};
use app::example::presentation::user::GetUserResponse;
use app::route::{auth_routes, public_routes};
use shared::external::database::{connect_test_db, ConnectionFactoryImpl};
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
