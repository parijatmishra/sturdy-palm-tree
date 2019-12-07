use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::get;

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello, again!")
}

// alternative
// using macro attributes to define routes
#[get("/hello")]
fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            .service(index3) // macro defined
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}