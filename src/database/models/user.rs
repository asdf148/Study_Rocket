use super::super::schema::user::users;
#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[table_name = "users"]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub password: String,
}