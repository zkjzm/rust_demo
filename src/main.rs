use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder ,HttpRequest, Error};
use serde::{Deserialize, Serialize};
use futures::future::{ready, Ready};

#[derive(Serialize)]
struct Oltdto {
    id: String,
    tag: String,
}
 
// Responder
impl Responder for Oltdto {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;
    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // 创建响应并设置Content-Type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[post("/olt")]
async fn olt(req_body: String)-> impl Responder{
    Oltdto {id : "11111".to_string(), tag : req_body}
}
#[derive(Deserialize)]
struct Info {
    id: String,
    tag: String,
    username: String,
}
// 仅仅在请求查询参数中有 'username' 字段时才会被调用
#[post("/query")]
async fn index(info: web::Json<Info>) -> String {
    format!("Welcome {}{}{}!", info.id,info.tag,info.username)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(olt)
            .service(index)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}