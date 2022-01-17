use actix_ratelimit::{MemoryStore, MemoryStoreActor, RateLimiter};
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use chrono::{DateTime, Local};
use json::JsonValue;
use serde::{Deserialize, Serialize};
use std::time::Duration;

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
            json::object! {"msg": "ok"}
        }
        Err(e) => json::object! {"err" => e.to_string() },
    };

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(injson.dump()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mem_store = MemoryStore::new();

    HttpServer::new(move || {
        App::new()
            // Register the middleware
            // which allows for a maximum of
            // 100 requests per minute per client
            // based on IP address
            .wrap(
                RateLimiter::new(MemoryStoreActor::from(mem_store.clone()).start())
                    .with_interval(Duration::from_secs(1))
                    .with_max_requests(1)
                    .with_identifier(|req| {
                        let content_type = req.headers().get("Content-Type").unwrap();
                        let content_type = content_type.to_str().unwrap();
                        println!("Content-Type is {}", content_type.to_string());
                        Ok(content_type.to_string())
                    }),
            )
            .route("/", web::post().to(index))
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/api/").route(web::post().to(api)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

async fn index() -> HttpResponse {
    let res = json::object! {
        "message": "tiny-runner is running...",
        "date": now()
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .body(res.dump())
}

fn now() -> String {
    let local_now: DateTime<Local> = Local::now();

    local_now.format("%Y-%m-%d- %H:%M:%S %Z").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test, App};

    #[actix_rt::test]
    async fn test_index_post_ok() {
        let mut app = test::init_service(App::new().route("/", web::post().to(index))).await;

        let req = test::TestRequest::post().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_api_post_ok() {
        let mut app = test::init_service(App::new().route("/api/", web::post().to(api))).await;

        let req_body = r#"{"uid":"123", "pw":"abc123"}"#;
        let req = test::TestRequest::post()
            .set_json(&req_body)
            .uri("/api/")
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
