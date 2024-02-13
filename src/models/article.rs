use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::articles;
use crate::db_connection::establish_connection;
use crate::schema::articles::dsl::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Article {
    pub fn find(article_id: &i32) -> Result<Article, diesel::result::Error> {
        let mut connection = establish_connection(); // diesel::pg::PgConnection is treated as mutable
        articles::table
            .find(article_id)
            .first(&mut connection) // diesel::pg::PgConnection is treated as mutable
    }

    pub fn destroy(article_id: &i32) -> Result<(), diesel::result::Error> {
        let mut connection = establish_connection(); // diesel::pg::PgConnection is treated as mutable
        diesel::delete(
            articles
                .find(article_id)
        )
        .execute(&mut connection)?;
        Ok(())
    }
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = articles)]
pub struct NewArticle {
    pub title: Option<String>,
    pub body: Option<String>,
}

impl NewArticle {
    pub fn create(&self) -> Result<Article, diesel::result::Error> {
        let mut connection = establish_connection();
        diesel::insert_into(articles::table)
            .values(self)
            .get_result(&mut connection)
    }
}

#[derive(Serialize, Deserialize)] 
pub struct ArticleList(pub Vec<Article>);

impl ArticleList {
    pub fn list() -> Self {
        let mut connection = establish_connection(); // diesel::pg::PgConnection is treated as mutable

        let result = articles
                        .limit(10)
                        .load::<Article>(&mut connection) // diesel::pg::PgConnection is treated as mutable
                        .expect("Error loading articles");

        ArticleList(result)
    }
}