#[cfg(test)]
mod tests{

  use actix_web::{http::StatusCode, web::Json};
  use crate::routes::*;
  use crate::models::QueryWord;

  #[actix_rt::test]
  async fn test_index(){

    let resp = index().await;

    assert_eq!(resp.status(), StatusCode::OK, 
    "Check Used port is not used by other application \
    or this this application config");
    
  }

  #[actix_rt::test]
  async fn test_json_query_page_meaning_not_found(){


    let info = QueryWord{
      word: "ejgiofdpsoigs8943t34543".to_string()
    };

    let info_json = Json::<QueryWord>(info);
    let resp = query_meaning(info_json).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

  }

  #[test]
  fn test_database_insertion() {
    // let mut conn = establish_connection();
    // let new_meaning = NewMeaning{
    //   word: "some_unknown_word",
    //   def: vec!["Unknown", "Definition not known"],
    //   keywords: vec!["unknown"]
    // };

  //   use crate::schema::meaning::dsl::meaning;
  //   use crate::schema::definition::dsl::definition;

  //   conn.test_transaction::<_, Error, _>(|_| {
  //     diesel::insert_into(meaning)
  //     .values(new_meaning)
  //     .execute(&mut conn).ok();

  //     Ok(())
  //   });

  //   let new_def = NewDefinition{
  //     word: "some_unknown_word",
  //     meaning_id: &1i32,
  //     synonyms: vec!["if any"],
  //     antonyms: vec!["if any"],
  //   };

  //   conn.test_transaction::<_, Error, _>(|_| {
  //     diesel::insert_into(definition)
  //     .values(new_def)
  //     .execute(&mut conn).ok();

  //     Ok(())
  //   });

  // }

  // #[test]
  // fn test_database_deletion(){
  //   let mut conn = establish_connection();

  //   use crate::schema::meaning::dsl::{meaning, meaning_id};
  //   use crate::schema::definition::dsl::{definition, word_id};
  //   use diesel::prelude::*;

  //   conn.test_transaction::<_, Error, _>(|_| {
  //     diesel::delete(meaning)
  //     .filter(meaning_id.eq(1))
  //     .execute(&mut conn).ok();

  //     Ok(())
  //   });

  //   conn.test_transaction::<_, Error, _>(|_| {
  //     diesel::delete(definition)
  //     .filter(word_id.eq(1))
  //     .execute(&mut conn).ok();

  //     Ok(())
  //   });

  }
}
