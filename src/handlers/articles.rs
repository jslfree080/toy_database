use actix_web::{ HttpResponse, web, Error };
use crate::models::article::ArticleList;
use crate::models::article::NewArticle;
use crate::models::article::Article;
use crate::db_connection::{ PgPool, PgPooledConnection };

fn pg_pool_handler(pool: web::Data<PgPool>) -> Result<PgPooledConnection, Error> {
    pool
    .get()
    .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))
}

pub async fn index(pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let mut pg_pool = pg_pool_handler(pool)?;
    Ok(HttpResponse::Ok().json(ArticleList::list(&mut *pg_pool)))
}

pub async fn create(new_article: web::Json<NewArticle>, pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let mut pg_pool = pg_pool_handler(pool)?;
    new_article
        .into_inner()
        .create(&mut *pg_pool)
        .map(|article| HttpResponse::Ok().json(article))
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))
}

pub async fn show(article_id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let mut pg_pool = pg_pool_handler(pool)?;
    Article::find(&article_id, &mut *pg_pool)
        .map(|article| HttpResponse::Ok().json(article))
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))
} 

pub async fn destroy(article_id: web::Path<i32>, pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let mut pg_pool = pg_pool_handler(pool)?;
    Article::destroy(&article_id, &mut *pg_pool)
        .map(|_| HttpResponse::Ok().json(()))
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))
}

pub async fn update(article_id: web::Path<i32>, new_article: web::Json<NewArticle>, pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let mut pg_pool = pg_pool_handler(pool)?;
    Article::update(&article_id, &new_article, &mut *pg_pool)
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
// {"id":3,"title":"Third article","body":"This is an attempt to update the previous article.","published":false}
*/