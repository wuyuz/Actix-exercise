use actix_web::{web, App, FromRequest,HttpServer};

/// extract text data from request
pub async fn other(text: String) -> String {
    format!("Body {}!", text)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(
            web::resource("/index")
                .app_data(String::configure(|cfg| {  // <- limit size of the payload
                    cfg.limit(4096)
                }))
                .route(web::get().to(other))  // <- register handler with extractor params
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 