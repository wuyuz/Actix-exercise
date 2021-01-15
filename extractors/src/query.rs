use actix_web::{get,web,App,HttpServer,Result,HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info{
    username: String
}

// this handler get called only if the request's query contains `username` field
#[get("/")]
async fn query_info(info: web::Query<Info>) -> String {
    format!("Welcome {}!", info.username)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(query_info))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}