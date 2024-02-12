use actix_web::{ Responder, HttpResponse };
use crate::models::article::ArticleList;

pub async fn index() -> impl Responder {
    HttpResponse::Ok().json(ArticleList::list())
}

/*
psql -U postgres -d toy_database -c "INSERT INTO articles(title, body) VALUES ('First article', 'This is my first article.'); INSERT INTO articles(title, body) VALUES ('Second article', 'What is inside of http://localhost:8080/articles ?');"
*/