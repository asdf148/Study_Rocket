#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(User)]
#[belongs_to(Post)]
pub struct Comment {
  pub id: i32,
  pub comment: String,
  pub user_id: i32,
  pub post_id: i32,
}