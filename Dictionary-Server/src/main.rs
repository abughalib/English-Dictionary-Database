pub mod models;
pub mod schema;
pub mod vars;
pub mod database_op;
pub mod tests;
pub mod routes;
pub mod populate_postgres;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate actix_rt;

use actix_web::{App, HttpResponse, HttpServer, guard, web};
use database_op::establish_connection;
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use crate::database_op::get_result;

#[derive(Deserialize, Serialize)]
struct WordQuery{
  word: String,
}

#[derive(Serialize, Debug)]
struct DefinitionResponse{
  word: String,
  meaning: Meaning,
  synonyms: Vec<String>,
  antonyms: Vec<String>,
}
#[derive(Serialize, Debug)]
struct Meaning{
  def: Vec<String>,
  keywords: Vec<String>,
}

async fn query_meaning(info: web::Json<WordQuery>)->HttpResponse{

  let conn = establish_connection();

  let result = get_result(&conn, (*info.word.to_uppercase()).to_string());

  match result{
    Ok(def)=>{
      let def_resp = DefinitionResponse{
        word: def.0.word,
        meaning: Meaning{
          def: def.1.def,
          keywords: def.1.keywords
        },
        synonyms: def.0.synonyms,
        antonyms: def.0.antonyms
      };
      HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("{:?}", def_resp))
    },
    Err(e)=>{
      return HttpResponse::NotFound().body(String::from(format!("{}", e)))
    }
  }
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
  HttpServer::new(|| {
    App::new()
        .wrap(
          Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600)
        )
    .route("/", web::get().to(routes::index))
      .service(web::scope("/dict")
        .guard(guard::Header("content-type", "application/json"))
        .route("api", web::post().to(query_meaning))
    )
  })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await

}