use actix_web::{
    http::StatusCode,
    test::{self, read_body},
    web::{self, Data},
    App,
};
use app::route::{auth_routes, public_routes};
use serde_json::Value;
use shared::{common::injector::Injector, external::database::ConnectionFactoryImpl};

#[actix_web::test]
async fn test_get_user() {
    // arrange
    let app = test::init_service(
        App::new()
            .service(
                web::scope("/api").configure(public_routes).service(
                    web::scope("")
                        // .wrap(auth_middleware)
                        .configure(auth_routes),
                ),
            )
            .app_data(Data::new(Injector::new(ConnectionFactoryImpl))),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/api/users/ca1c5eb2-a43a-4cef-80fc-f9abe1623785"))
        .to_request();

    // act
    let resp = test::call_service(&app, req).await;

    // assert
    assert_eq!(resp.status(), StatusCode::OK);

    let resp_body = read_body(resp).await;
    let expected = r#"{
        "id": "ca1c5eb2-a43a-4cef-80fc-f9abe1623785",
        "name": "test name"
    }"#
    .as_bytes();

    assert_eq!(
        serde_json::from_slice::<Value>(&resp_body).unwrap(),
        serde_json::from_slice::<Value>(expected).unwrap(),
    );
}
