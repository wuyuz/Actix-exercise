mod diff_response;

use crate::diff_response::either;
use actix_web::{Error,HttpResponse,Responder,web,App,HttpServer};
use futures::{ future::ok, stream::once }; 

async fn index() -> impl Responder {
    // ok: Create a future that is immediately ready with a success value.
    // once: Creates a stream of a single element.
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/either", web::get().to(either))
        
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}