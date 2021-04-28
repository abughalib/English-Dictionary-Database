pub mod models;
pub mod schema;
pub mod vars;
pub mod database_op;
pub mod tests;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate actix_rt;

use actix_web::{App, HttpResponse, HttpServer, web};

async fn index()->HttpResponse{
  HttpResponse::Ok().body("<h1>Index Page</h1>")
}

#[actix_web::main]
async fn main()->std::io::Result<()>{
  HttpServer::new(|| {
    App::new()
    .route("/", web::get().to(index))
  })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}