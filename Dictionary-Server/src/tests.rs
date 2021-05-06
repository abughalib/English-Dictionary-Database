#[cfg(test)]
mod tests{

  use crate::{database_op::{get_def, insert_meaning}, models::Definition};
  use crate::{database_op::{delete_word, establish_connection, insert_definition}, models::{NewMeaning, NewDefinition}};

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
    let new_meaning = NewMeaning{
      def: vec!["Noun", "A fruit grows in cold area"],
      keywords: vec!["fruit"]
    };

    let new_mean = NewMeaning{
      def: vec!["Noun", "Some meaning here"],
      keywords: vec!["test", "test_word"]
    };

    let index: i32 = insert_meaning(&conn, new_mean)
      .ok().expect("Failed to insert meaning") as i32;

    let new_def = NewDefinition{
      word: "test_word",
      meaning_id: &index,
      synonyms: vec!["somthing must be"],
      antonyms: vec![""],
    };

    assert_eq!(insert_definition(&conn, new_def).is_ok(), true);
    assert_eq!(insert_meaning(&conn, new_meaning).is_ok(), true);

    let get_word: Definition = get_def(&conn, "test_word".to_string())
      .ok().unwrap()[0].clone();

    assert_eq!(delete_word("test_word".to_string()).is_ok(), true);
    assert_eq!(get_word.word, "test_word".to_string());

  }

  #[test]
  fn test_database_deletion(){
    let conn = establish_connection();

    let new_mean = NewMeaning{
      def: vec!["Noun", "Some meaning here"],
      keywords: vec!["test", "test_word"]
    };

    let index: i32 = insert_meaning(&conn, new_mean)
      .ok().expect("Failed to insert meaning") as i32;

    let new_def = NewDefinition{
      meaning_id: &index,
      word: "test_word",
      antonyms: vec![],
      synonyms: vec![]
    };

    assert_eq!(insert_definition(&conn, new_def).is_ok(), true);
    assert_eq!(delete_word("test_word".to_string()).is_ok(), true);

    let get_word= get_def(&conn, "test_word".to_string());
    assert_eq!(get_word.ok().unwrap().len(), 0);

  }
}