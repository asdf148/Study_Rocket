use super::service;

pub fn join() -> &'static str {
  service::create();
  "Hello, world!"
}