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
  async fn test_json_query_page_meaning_not_found(){
    
    let info = WordQuery{
      word: "u4893754tgjkhdfu".to_string()
    };

    let info_json = Json::<WordQuery>(info);
    let resp = query_meaning(info_json).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

  }
  #[actix_rt::test]
  async fn test_json_query_page_meaning_found(){
    
    let info = WordQuery{
      word: "u4893754tgjkhdfu".to_string()
    };

    let conn = establish_connection();
    let new_meaning = NewMeaning{
      word: "u4893754tgjkhdfu",
      def: vec!["Unknown", "Definition not known"],
      keywords: vec!["unknown"]
    };

    let index: i32 = insert_meaning(&conn, new_meaning)
      .ok().expect("Failed to insert meaning");

    let new_def = NewDefinition{
      word: "u4893754tgjkhdfu",
      meaning_id: &index,
      synonyms: vec!["if any"],
      antonyms: vec!["if any"],
    };

    assert_eq!(insert_definition(&conn, new_def).is_ok(), true);

    let info_json = Json::<WordQuery>(info);
    let resp = query_meaning(info_json).await;
    assert_eq!(resp.status(), StatusCode::OK);

    assert_eq!(delete_word("u4893754tgjkhdfu".to_string()).is_ok(), true);

  }

  #[test]
  fn test_database_insertion(){
    let conn = establish_connection();
    let new_meaning = NewMeaning{
      word: "some_unknown_word",
      def: vec!["Unknown", "Definition not known"],
      keywords: vec!["unknown"]
    };

    let index: i32 = insert_meaning(&conn, new_meaning)
      .ok().expect("Failed to insert meaning");

    let new_def = NewDefinition{
      word: "some_unknown_word",
      meaning_id: &index,
      synonyms: vec!["if any"],
      antonyms: vec!["if any"],
    };

    assert_eq!(insert_definition(&conn, new_def).is_ok(), true);

    let get_word: Definition = get_def(&conn, "some_unknown_word".to_string())
      .ok().unwrap()[0].clone();

    assert_eq!(delete_word("some_unknown_word".to_string()).is_ok(), true);
    assert_eq!(get_word.word, "some_unknown_word".to_string());

  }

  #[test]
  fn test_database_deletion(){
    let conn = establish_connection();

    let new_meaning = NewMeaning{
      word: "some_unknown_2",
      def: vec!["Unknown", "Definition not known"],
      keywords: vec!["unknown"]
    };

    let index: i32 = insert_meaning(&conn, new_meaning)
      .ok().expect("Failed to insert meaning");

    let new_def = NewDefinition{
      word: "some_unknown_2",
      meaning_id: &index,
      synonyms: vec!["if any"],
      antonyms: vec!["if any"],
    };

    assert_eq!(insert_definition(&conn, new_def).is_ok(), true);
    assert_eq!(delete_word("some_unknown_2".to_string()).is_ok(), true);

    let get_word= get_def(&conn, "some_unknown_2".to_string());
    assert_eq!(get_word.ok().unwrap().len(), 0);

  }
}