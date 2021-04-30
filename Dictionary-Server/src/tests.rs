#[cfg(test)]
mod tests{

  use crate::{database_op::{delete_word, establish_connection, insert_definition}, models::{Definition, NewDefinition}};

use super::super::*;
  use super::super::routes::*;

  use actix_web::{http::StatusCode, web::Json};
  #[actix_rt::test]
  async fn test_index(){

    let resp = index().await;

    assert_eq!(resp.status(), StatusCode::OK, 
    "Check Used port is not used by other application \
    or this this application config");
    
  }

  #[actix_rt::test]
  async fn test_test_query_page(){
    
    let info = WordQuery{
      word: "apple".to_string()
    };

    let info_json = Json::<WordQuery>(info);
    let resp = query_meaning(info_json).await;
    assert_eq!(resp.status(), StatusCode::OK);
  
  }
  #[test]
  fn test_database_insertion(){
    let conn = establish_connection();
    let new_def = NewDefinition{
      word: "test_word",
      meaning: Some(vec!["Some meaning"]),
      synonyms: Some(vec!["somthing must be"]),
      antonyms: Some(vec![""]),
    };

    assert_eq!(insert_definition(&conn, new_def).is_ok(), true);

    let get_word: Definition = get_result(&conn, "test_word".to_string())
      .ok().unwrap()[0].clone();

    assert_eq!(delete_word("test_word".to_string()).is_ok(), true);
    assert_eq!(get_word.word, "test_word".to_string());

  }

  #[test]
  fn test_database_deletion(){
    let conn = establish_connection();
    let new_def = NewDefinition{
      word: "test_word",
      meaning: Some(vec!["Some meaning"]),
      synonyms: Some(vec!["somthing must be"]),
      antonyms: Some(vec![""]),
    };
    assert_eq!(insert_definition(&conn, new_def).is_ok(), true);
    assert_eq!(delete_word("test_word".to_string()).is_ok(), true);

    let get_word= get_result(&conn, "test_word".to_string());
    assert_eq!(get_word.ok().unwrap().len(), 0);

  }
}