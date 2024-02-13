use actix_web::{ Responder, HttpResponse, web, Error };
use crate::models::article::ArticleList;
use crate::models::article::NewArticle;
use crate::models::article::Article;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(ArticleList::list())
}

pub async fn create(new_article: web::Json<NewArticle>) -> Result<HttpResponse, Error> {
    new_article
        .into_inner()
        .create()
        .map(|article| HttpResponse::Ok().json(article))
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))
}

pub async fn show(article_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Article::find(&article_id)
        .map(|article| HttpResponse::Ok().json(article))
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))
} 

pub async fn destroy(article_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    Article::destroy(&article_id)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))
}

pub async fn update(article_id: web::Path<i32>, new_article: web::Json<NewArticle>) -> Result<HttpResponse, Error> {
    Article::update(&article_id, &new_article)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))
}

/*
psql -U postgres -d toy_database -c "INSERT INTO articles(title, body) VALUES ('First article', 'This is my first article.'); INSERT INTO articles(title, body) VALUES ('Second article', 'What is inside of http://localhost:8080/articles ?');"
*/

/*
cargo run
curl http://127.0.0.1:8080/articles -H "Content-Type: application/json" -d '{"title": "Third article", "body": "This is an attempt to create a new article."}'
// {"id":3,"title":"Third article","body":"This is an attempt to create a new article.","published":false}
*/

/*
cargo run
curl -X DELETE http://127.0.0.1:8080/articles/1 -H "Content-Type: application/json"
// Record not found
*/

/*
cargo run
curl -X PATCH http://127.0.0.1:8080/articles/3 -H "Content-Type: application/json" -d '{"body": "This is an attempt to update the previous article."}'
*/