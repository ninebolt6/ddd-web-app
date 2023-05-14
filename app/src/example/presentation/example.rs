use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use share::common::result::ResponseResult;

pub fn example_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/hello").route(web::get().to(hello)))
        .service(web::resource("/hello/{text}").route(web::post().to(post_hello)));
}

async fn hello() -> ResponseResult {
    Ok(HttpResponse::Ok().body("Hello world!"))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PostHelloRequest {
    user_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
struct PostHelloResponse {
    message: String,
    text: String,
}

async fn post_hello(path: web::Path<String>, body: web::Json<PostHelloRequest>) -> ResponseResult {
    let path_param = path.into_inner();
    let user_name = &body.user_name;

    Ok(HttpResponse::Ok().json(PostHelloResponse {
        message: format!("Hello, {user_name}!"),
        text: path_param.repeat(2),
    }))
}

