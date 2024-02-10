use actix_web::{Responder, HttpResponse };
use serde_json::to_string;
use crate::models::article::ArticleList;

pub async fn index() -> impl Responder {
    let article_list = ArticleList::list();
    match to_string(&article_list) {
        Ok(json) => HttpResponse::Ok()
                        .content_type("application/json")
                        .body(json),
        Err(_) => HttpResponse::InternalServerError()
                        .finish(),
    }
}

/*
psql -U postgres -d toy_database -c "INSERT INTO articles(title, body) VALUES ('First article', 'This is my first article.'); INSERT INTO articles(title, body) VALUES ('Second article', 'What is inside of http://localhost:8080/articles ?');"
*/