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

#[derive(Insertable)]
#[diesel(table_name = articles)]
pub struct NewArticle {
    pub title: Option<String>,
    pub body: Option<String>,
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