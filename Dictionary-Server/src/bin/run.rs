extern crate dotenv;
extern crate actix_rt;
extern crate dictionary_server;

use actix_web::{App, HttpServer, guard, web};
use dictionary_server::routes::{index, help, query_meaning, page_not_found};
use dictionary_server::vars::get_host_path;
use actix_cors::Cors;

#[actix_web::main]
async fn main()->std::io::Result<()>{
  HttpServer::new(|| {
    App::new()
    .wrap(
      Cors::default()
        .allow_any_origin()
        .allow_any_header()
        .allowed_methods(vec!["GET", "POST"])
        .max_age(3600)
    )
    .route("/", web::get().to(index))
    .route("/help", web::get().to(help))
    .service(web::scope("/dict")
    .guard(guard::Header("content-type", "application/json"))
    .route("api", web::post().to(query_meaning))
  )
  .default_service(
    web::route()
    .guard(guard::Not(guard::Get()))
    .to(page_not_found)
  )

  })
    .bind(get_host_path())?
    .run()
    .await

}