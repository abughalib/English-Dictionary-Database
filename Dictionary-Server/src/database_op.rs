use diesel::{dsl::exists, insert_into, pg::PgConnection, select};
use diesel::prelude::*;

use crate::models::{Definition, NewDefinition};

use super::vars::get_database_url;
use super::schema::definition::dsl::*;

pub fn establish_connection() -> PgConnection {
  PgConnection::establish(&get_database_url())
  .expect(&format!("Error Connecting to {}", get_database_url()))
}

pub fn insert_definition<'a>(conn: &PgConnection, new_def: NewDefinition<'a>)->Result<usize, String>{

  let word_exists = select(
    exists(definition.filter(word.eq(new_def.word)))
  ).get_result(conn);

  match word_exists{
    Ok(true)=>{
      return Err(String::from(format!("Cannot insert duplicate word")));
    },
    Ok(false)=>{
      match insert_into(definition)
        .values(new_def)
        .execute(conn){
        Ok(t)=>{
          return Ok(t)
        }
        Err(_)=>{
          return Err(String::from(format!("Cannot insert word")));
        }
      }
    }
    Err(_)=>{
      return Err(String::from(format!("Unknow Error")));
    }
  }
}

pub fn get_result(conn: &PgConnection, new_word: String)->Result<Vec<Definition>, String>{
  let def = definition
    .filter(word.eq(new_word))
    .limit(1)
    .load::<Definition>(conn);

  match def{
    Ok(t)=>{
      return Ok(t);
    }
    Err(_)=>{
      return Err(String::from(format!("Cannot find word")))
    }
  }
}

pub fn delete_word(new_word: String)->Result<usize, String>{
  let conn = establish_connection();
  match diesel::delete(definition
    .filter(word.eq(new_word)))
    .execute(&conn){
      Ok(t)=>{
        Ok(t)
      },
      Err(_)=>{
        Err(String::from(format!("Word not found")))
      }
    }

}