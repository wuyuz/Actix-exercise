use actix_web::{error, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct InfoJson {
    username: String,
}

/// deserialize `Info` from request's body, max payload size is 4kb
pub async fn json_info(info: web::Json<InfoJson>) -> impl Responder {
    format!("Welcome {}!", info.username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()  //  设置json提取器的配置
            .limit(4096) // 限制提取内容的大小
            .error_handler(|err, _req| {  // 自定义错误返回
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new().service(
            web::resource("/")
                // change json extractor configuration
                .app_data(json_config)
                .route(web::post().to(json_info)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}