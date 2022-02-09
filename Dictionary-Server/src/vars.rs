extern crate dotenv;
use std::env;
use dotenv::dotenv;

pub fn get_database_url()->String{
  dotenv().ok();
  let url = env::var("DB_URL")
  .expect("DATABASE_URL must be set to .env");

  return url;
}

pub fn get_host_path()-> (String, u16){
  dotenv().ok();
  let host = env::var("HOST")
    .expect("HOST not defined");
  
  let port = env::var("PORT")
    .expect("PORT not defined").parse()
    .ok().expect("PORT must be Unsigned Integer");

  return (host, port)
}