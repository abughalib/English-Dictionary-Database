use diesel::{dsl::exists, insert_into, pg::PgConnection, select};
use diesel::prelude::*;

use crate::{models::{Definition, NewDefinition, NewMeaning, Meaning}};

use super::vars::get_database_url;
use super::schema::definition::dsl::*;
use super::schema::meaning::dsl::*;

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

pub fn insert_meaning(conn: &PgConnection, new_meaning: NewMeaning)->Result<usize, String>{
  match insert_into(meaning)
    .values(new_meaning)
    .execute(conn){
      Ok(t)=>return Ok(t),
      Err(_)=>{}
    }
  Err("Unknown Error".to_string())
}

pub fn get_def(conn: &PgConnection, new_word: String)->Result<Vec<Definition>, String>{
  let defn = definition
    .filter(word.eq(new_word))
    .limit(1)
    .load::<Definition>(conn);

  match defn{
    Ok(t)=>{
      return Ok(t);
    }
    Err(_)=>{
      return Err(String::from(format!("Cannot find word")))
    }
  }
}

pub fn get_mean(conn: &PgConnection, mean_id: i32)->Result<Vec<Meaning>, String>{
  let def_mean = meaning
    .filter(id.eq(mean_id))
    .limit(1)
    .load::<Meaning>(conn);

  match def_mean{
    Ok(t)=>{
      return Ok(t)
    },
    Err(_)=>{
      return Err(String::from(format!("Cannot find meaning")))
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

pub fn get_result(conn: &PgConnection, new_word: String)->Result<(Definition, Meaning), String>{
  let defin = get_def(conn, new_word)?;

  if defin.len() == 0{
    return Err("Cannot find definition".to_string())
  }
  let mean = get_mean(conn, defin[0].meaning_id)?;
  
  return Ok((defin[0].clone(), mean[0].clone()));
}