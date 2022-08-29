pub mod models;
pub mod schema;

use diesel::{MysqlConnection, Connection};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
  dotenv().ok();

  let database_url: String = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");

  MysqlConnection::establish(&database_url)
    .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}