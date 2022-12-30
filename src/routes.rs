use super::database_op::establish_connection;
use super::database_op::get_result;
use super::models::*;
use actix_web::{web, HttpResponse};

pub async fn index() -> HttpResponse {
    HttpResponse::Ok().body("<h1>For Help: Get Request /help</h1>")
}

pub async fn help() -> HttpResponse {
    let style = String::from(
        "
    <style>
      .uses{
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-text: center;
        font-size: 30px;
      }
      .uses-title{
        background: black;
        color: white;
        width: auto;
      }
    </style>
  ",
    );

    let reponse = String::from(
        r#"
    <div class='uses'>
      <h1 class='uses-title'>API USES</h1>
      POST: /dict/api<br>
      body with json data.<br>
      {
        <br>
        <span>  </span>'word': 'word'<br>
      }<br>
    </div>
  "#,
    );

    HttpResponse::Ok()
        .content_type("text/html")
        .body(style + &reponse)
}

pub async fn query_meaning(info: web::Json<QueryWord>) -> HttpResponse {
    let mut conn = establish_connection();

    let result = get_result(&mut conn, (*info.word.to_uppercase()).to_string());

    match result {
        Ok(def) => {
            let def_resp = QueryDefinition {
                word: def.0.word,
                meaning: QueryMeaning {
                    def: def.1.def,
                    keywords: def.1.keywords,
                },
                synonyms: def.0.synonyms,
                antonyms: def.0.antonyms,
            };
            HttpResponse::Ok()
                .content_type("application/json")
                .json(web::Json(def_resp))
        }
        Err(_) => {
            return HttpResponse::NotFound().body(String::from(format!(
                "Cannot find meaning of {}",
                info.word
            )))
        }
    }
}

pub async fn page_not_found() -> HttpResponse {
    let resp = r#"
    <style>
    body{
      font-family: 'Merriweather', serif;
      margin: 0;
      background-color: #9cc3d5;
      text-align: center;
      color: white;
      user-select: none;
      padding-top: 18vh;
  }
  .container{
      display: flex;
      flex-direction: column;
      align-items: center;
      width: 100%;
      background-color: #5ca3dd93;
  }
  h2{
      font-size: 150px;
      margin: 0;
      text-shadow: 15px 5px 2px black;
  }
  h3{
      font-size: 40px;
      margin: 20px;
  }
  p{
      font-size: 18px;
      margin: 5px;
  }
  p:last-of-type{
      margin-bottom: 35px;
  }
  a{
      text-decoration: none;
      
  }
    </style>
    <div class="container">
        <h2>404</h2>
        <h3>Oops, nothing here...</h3>
        <h3>Go Back <a href="/help">Click Here</a></h3>
    </div>
  "#;

    HttpResponse::NotFound().body(resp)
}
