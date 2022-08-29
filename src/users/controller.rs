use rocket::http::ContentType;
use rocket::Data;
use rocket_contrib::json::Json;
use serde::json::{Json};
use serde::{Deserialize, Serialize};

use super::dto::user_dto;
// use super::service;

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomResponse {
  pub message: String,
}

#[post("/join", format="json", data = "<user_data>")]
pub fn join(user_data: Json<UserDTO>) -> Json<str> {
  // let mut test = Data
  Json(user_data);
}