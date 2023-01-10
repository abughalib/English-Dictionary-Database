extern crate actix_rt;
extern crate dictionary_server;
extern crate dotenv;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpServer};
use dictionary_server::routes::{help, index, page_not_found, query_meaning, query_words};
use dictionary_server::vars::get_host_path;
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allowed_methods(vec!["GET", "POST"])
                    .max_age(3600),
            )
            .route("/", web::get().to(index))
            .route("/help", web::get().to(help))
            .service(
                web::scope("/dict")
                    .guard(guard::Header("content-type", "application/json"))
                    .route("api", web::post().to(query_meaning))
                    .guard(guard::Header("content-type", "application/json"))
                    .route("search", web::post().to(query_words)),
            )
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(page_not_found),
            )
    })
    .bind(get_host_path())?
    .run()
    .await
}
