use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use toy_database::db_connection::establish_connection;
use toy_database::handlers;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
#[allow(deprecated)]
async fn main() -> std::io::Result<()> {
    println!("Starting Rust Actix-web server at 127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .data(establish_connection())
            .service(hello)
            .service(
                web::resource("/articles")
                    .route(web::get().to(handlers::articles::index)) // http://127.0.0.1:8080/articles
                    .route(web::post().to(handlers::articles::create)),
            )
            .service(
                web::resource("/articles/{article_id}")
                    .route(web::get().to(handlers::articles::show)) // http://127.0.0.1:8080/articles/3
                    .route(web::delete().to(handlers::articles::destroy))
                    .route(web::patch().to(handlers::articles::update)), // http://127.0.0.1:8080/articles/3
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
