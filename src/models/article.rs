use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::articles;
use crate::db_connection::establish_connection;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = articles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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

        use crate::schema::articles::dsl::*;

        let mut connection = establish_connection();

        let result = 
            articles
                .limit(10)
                .load::<Article>(&mut connection)
                .expect("Error loading articles");

        ArticleList(result)
    }
}