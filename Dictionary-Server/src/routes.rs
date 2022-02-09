use actix_web::{HttpResponse, web};
use super::models::*;
use super::database_op::establish_connection;
use super::database_op::get_result;

pub async fn index()->HttpResponse{
  HttpResponse::Ok().body("<h1>For Help: Get Request /help</h1>")
}

pub async fn help()->HttpResponse{
  
  let style = String::from("
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
  ");
  
  let reponse = String::from(r#"
    <div class='uses'>
      <h1 class='uses-title'>API USES</h1>
      POST: /dict/api<br>
      body with json data.<br>
      {
        <br>
        <span>  </span>'word': 'word'<br>
      }<br>
    </div>
  "#);

  HttpResponse::Ok().content_type("text/html").body(style+&reponse)
}

pub async fn query_meaning(info: web::Json<QueryWord>)->HttpResponse{

  let conn = establish_connection();

  let result = get_result(&conn, 
    (*info.word.to_uppercase()).to_string());

  match result{
    Ok(def)=>{
      let def_resp = QueryDefinition{
        word: def.0.word,
        meaning: QueryMeaning{
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