use super::super::schema::post::posts;
use super::user::User;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User)]
#[table_name = "posts"]
pub struct Post {
  pub id: i32,
  pub image: String,
  pub title: String,
  pub content: String,
  pub user_id: i32,
}