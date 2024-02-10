use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use toy_database::handlers;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting at 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/articles", web::get().to(handlers::articles::index)) // http://127.0.0.1:8080/articles
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}