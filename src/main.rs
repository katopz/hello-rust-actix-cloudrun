use std::env;

use actix_web::{
    get, post,
    web::{self, Query},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};

use serde::Deserialize;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Foo {
    id: String,
}

#[get("/search")]
async fn search(req: HttpRequest) -> impl Responder {
    let query = req.query_string();
    match Query::<Foo>::from_query(query) {
        Ok(foo) => HttpResponse::Ok().body(format!("Hi! {foo:?}")),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[get("/bar")]
async fn bar() -> impl Responder {
    HttpResponse::Ok().body("Hello bar!")
}

async fn manual_hello() -> impl Responder {
    // Test env "TARGET" which defined when `docker run`, or `gcloud run deploy --set-env-vars`
    // Depend on your platform target. (See README.md)
    let test_target = match env::var("TARGET") {
        Ok(target) => format!("Hey {target}!"),
        Err(_e) => "No TARGET env defined!".to_owned(),
    };

    // Response with test_target
    HttpResponse::Ok().body(test_target)
}

async fn get_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // "/"
            .service(hello)
            // "/echo"
            .service(echo)
            // "/hey"
            .route("/hey", web::get().to(manual_hello))
            .service(
                // "/api"
                web::scope("/api").service(
                    // "/v1"
                    web::scope("/v1")
                        // "/api/v1/search?id=bar"
                        .service(search)
                        // "/api/v1/bar"
                        .service(bar),
                ),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    get_server().await
}

#[cfg(test)]
mod tests {
    use crate::hello;
    use actix_web::App;

    #[actix_rt::test]
    async fn test() {
        let srv = actix_test::start(|| App::new().service(hello));

        let req = srv.get("/");
        let response = req.send().await.unwrap();
        assert!(response.status().is_success());
    }
}
