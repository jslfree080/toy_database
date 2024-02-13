use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use toy_database::handlers;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Rust Actix-web server at 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .route("/articles", web::get().to(handlers::articles::index)) // http://127.0.0.1:8080/articles
            .route("/articles", web::post().to(handlers::articles::create))
            .route("/articles/{article_id}", web::get().to(handlers::articles::show)) // http://127.0.0.1:8080/articles/3
            .route("/articles/{article_id}", web::delete().to(handlers::articles::destroy))
            .route("/articles/{article_id}", web::patch().to(handlers::articles::update)) // http://127.0.0.1:8080/articles/3
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}