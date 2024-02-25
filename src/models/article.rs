use crate::schema::articles;
use crate::schema::articles::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Article {
    pub fn find(
        article_id: &i32,
        connection: &mut PgConnection,
    ) -> Result<Article, diesel::result::Error> {
        articles::table.find(article_id).first(connection) //
    }

    pub fn destroy(
        article_id: &i32,
        connection: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::delete(articles.find(article_id)).execute(connection)?; //
        Ok(())
    }

    pub fn update(
        article_id: &i32,
        new_article: &NewArticle,
        connection: &mut PgConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::update(articles.find(article_id))
            .set(new_article)
            .execute(connection)?; //
        Ok(())
    }
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[diesel(table_name = articles)]
pub struct NewArticle {
    pub title: Option<String>,
    pub body: Option<String>,
}

impl NewArticle {
    pub fn create(&self, connection: &mut PgConnection) -> Result<Article, diesel::result::Error> {
        diesel::insert_into(articles::table)
            .values(self)
            .get_result(connection) //
    }
}

#[derive(Serialize, Deserialize)]
pub struct ArticleList(pub Vec<Article>);

impl ArticleList {
    pub fn list(connection: &mut PgConnection) -> Self {
        let result = articles
            .limit(10)
            .load::<Article>(connection) //
            .expect("Error loading articles");

        ArticleList(result)
    }
}
