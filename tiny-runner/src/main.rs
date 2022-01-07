use actix_web::{web, App, Error, HttpResponse, HttpServer};
use json::JsonValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

/// This handler uses json extractor
async fn api(body: web::Bytes) -> Result<HttpResponse, Error> {
    let result = json::parse(std::str::from_utf8(&body).unwrap());

    let injson: JsonValue = match result {
        Ok(_) => {
            json::object! {"api": ""}
        }
        Err(e) => json::object! {"err" => e.to_string() },
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(injson.dump()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/api/").route(web::post().to(api)))
            .service(web::resource("/").route(web::post().to(index)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#" {"message": "tiny-runner is running..."} "#)
}
