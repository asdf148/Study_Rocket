use super::super::schema::comment::comments;
use super::user::User;
use super::post::Post;

use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Comment {
  pub id: i32,
  pub comment: String,
  pub user_id: i32,
  pub post_id: i32,
}

#[derive(Debug, Insertable, Associations, Serialize)]
#[belongs_to(User)]
#[belongs_to(Post)]
#[table_name = "comments"]
pub struct NewComment {
  pub id: i32,
  pub comment: String,
  pub user_id: i32,
  pub post_id: i32,
}