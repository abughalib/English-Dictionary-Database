extern crate dotenv;
use std::env;
use dotenv::dotenv;

pub fn get_database_url()->String{
  dotenv().ok();
  let url = env::var("DB_URL")
  .expect("DATABASE_URL must be set to .env");

  let database_url = String::from(
    format!(
      "postgres://{}:{}@{}", get_database_user(), 
      get_database_password(), url
    ));
  return database_url;
}

pub fn get_database_user()->String{
  dotenv().ok();
  env::var("DB_USER")
    .expect("DATABASE_USER must be set in .env")
}

pub fn get_database_password()->String{
  dotenv().ok();
  env::var("DB_PASS")
    .expect("DATABASE_PASSWORD must be set in .env")
}