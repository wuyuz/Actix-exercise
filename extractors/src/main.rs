mod query;
mod json;
mod other;
use query::query_info;
use json::json_info;
use other::other;

use actix_web::{web, App, get, HttpServer,HttpRequest, Responder,HttpResponse,Result,error};
use serde::Deserialize;


#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}


// 路由匹配方式一
async fn index(web::Path((user_id, friend)): web::Path<(u32, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Welcome {}, user_id {}!", friend, user_id))
}

// 路由提取方式二： extract path info using serde
#[get("/users_2/{user_id}/{friend}")] // <- define path parameters
async fn index2(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.friend, info.user_id
    ))
}

// 路由提取方式三
#[get("/users_3/{userid}/{friend}")] // <- define path parameters
async fn index3(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("friend").unwrap().parse().unwrap();
    let userid: i32 = req.match_info().query("userid").parse().unwrap();

    Ok(format!("Welcome {}, userid {}!", name, userid))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()  //  设置json提取器的配置
        .limit(4096) // 限制提取内容的大小
        .error_handler(|err, _req| {  // 自定义错误返回
            // create custom error response
            println!("err:{}",err);
            error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
        });

        App::new()
            .route("/users_1/{user_id}/{friend}",web::post().to(index))
            .service(index2)
            .service(index3)
            .service(query_info)
            .service(
                web::resource("/json")
                // change json extractor configuration
                .app_data(json_config)
                .route(web::post().to(json_info)),
            )
            .service(
                web::resource("/index")
                    .route(web::get().to(other))  // <- register handler with extractor params
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
