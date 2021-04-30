use diesel::{insert_into, pg::PgConnection};
use diesel::prelude::*;

use crate::models::{Definition, NewDefinition};

use super::vars::get_database_url;
use super::schema::definition::dsl::*;

pub fn establish_connection() -> PgConnection {
  PgConnection::establish(&get_database_url())
  .expect(&format!("Error Connecting to {}", get_database_url()))
}

pub fn insert_definition<'a>(new_def: NewDefinition<'a>){
  
  let conn = establish_connection();

  match insert_into(definition)
    .values(new_def)
    .execute(&conn){
      Ok(t)=>{
        println!("{}", t);
      },
      Err(e)=>{
        println!("{}", e);
      }
    }
}


pub fn get_result(new_word: String)->Option<Vec<Definition>>{
  let conn = establish_connection();
  let result = definition
    .filter(word.eq(new_word))
    .limit(1)
    .load::<Definition>(&conn);

  match result{
    Ok(vec_def)=>{
      return Some(vec_def);
    },
    Err(_)=>{
      return None;
    }
  }
}

pub fn word_delete(new_word: String){
  let conn = establish_connection();
  diesel::delete(definition
    .filter(word.eq(new_word)))
    .execute(&conn)
    .expect("Error Deleting Posts");

}