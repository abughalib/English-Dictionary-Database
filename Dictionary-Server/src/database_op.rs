use diesel::{pg::PgConnection};
use diesel::prelude::*;

use super::vars::get_database_url;
use super::schema::definition::dsl::*;

pub fn establish_connection() -> PgConnection {
  PgConnection::establish(&get_database_url())
  .expect(&format!("Error Connecting to {}", get_database_url()))
}

// pub fn get_result<'a>(new_word: String)->Vec<Definition>{
//   let connection = establish_connection();
//   let results: Vec<Definition> = definition
//     .filter(word.eq(new_word))
//     .limit(1)
//     .load::<Definition>(&connection)
//     .expect("Error Loading Word Definitions");
//   results
// }

pub fn word_delete(new_word: String){
  let conn = establish_connection();
  diesel::delete(definition
    .filter(word.eq(new_word)))
    .execute(&conn)
    .expect("Error Deleting Posts");

}