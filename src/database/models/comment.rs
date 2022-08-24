use super::super::schema::comment::comments;
use super::user::User;
use super::post::Post;

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User)]
#[belongs_to(Post)]
#[table_name = "comments"]
pub struct Comment {
  pub id: i32,
  pub comment: String,
  pub user_id: i32,
  pub post_id: i32,
}