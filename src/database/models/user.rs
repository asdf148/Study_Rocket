use super::super::schema::user::users;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub password: String,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name="users"]
pub struct NewUser {
  pub name: String,
  pub email: String,
  pub password: String,
}