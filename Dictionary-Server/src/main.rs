pub mod models;
pub mod schema;
pub mod vars;
pub mod database_op;
pub mod tests;
pub mod routes;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate actix_rt;

use actix_web::{App, HttpServer, web, Result};
use serde::Deserialize;

use crate::database_op::get_result;


#[derive(Deserialize)]
struct WordQuery{
  word: String,
}

async fn query_meaning(info: web::Json<WordQuery>)->Result<String>{

  let result = get_result((*info.word).to_string());

  match result{
    Some(def)=>{
      match def.get(1){
        Some(t)=>{
          return Ok(format!("{:?}", t));
        },
        None=>{return Ok(format!("Meaning for {} is not available in our database", info.word));}
      }
    },
    None=>{
      return Ok(String::from("Some Unknown Error"))
    }
  }
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
  HttpServer::new(|| {
    App::new()
    .route("/", web::get().to(routes::index))
        .service(web::scope("/dict")
            .route("api", web::post().to(query_meaning))
      )
  })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}