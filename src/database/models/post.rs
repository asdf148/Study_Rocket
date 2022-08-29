use super::super::schema::post::posts;
use super::user::User;

use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Post {
  pub id: i32,
  pub image: String,
  pub title: String,
  pub content: String,
  pub user_id: i32,
}

#[derive(Debug, Insertable, Associations, AsChangeset)]
#[belongs_to(User)]
#[table_name = "posts"]
pub struct NewPost {
  pub id: i32,
  pub image: String,
  pub title: String,
  pub content: String,
  pub user_id: i32,
}