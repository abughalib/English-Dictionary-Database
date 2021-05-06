use actix_web::HttpResponse;

pub async fn index()->HttpResponse{
  HttpResponse::Ok().body("<h1>Index Page</h1>")
}

