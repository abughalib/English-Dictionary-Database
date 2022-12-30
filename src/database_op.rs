use diesel::prelude::*;
use diesel::result::Error;
use diesel::{dsl::exists, insert_into, pg::PgConnection, select};

use crate::models::{Definition, Meaning, NewDefinition, NewMeaning};

use super::vars::get_database_url;

pub fn establish_connection() -> PgConnection {
    PgConnection::establish(&get_database_url())
        .expect(&format!("Error Connecting to {}", get_database_url()))
}

pub fn insert_definition<'a>(
    conn: &mut PgConnection,
    new_def: NewDefinition<'a>,
) -> Result<(), String> {
    use super::schema::definition::dsl::*;

    let word_exists = select(exists(definition.filter(word.eq(new_def.word)))).get_result(conn);

    match word_exists {
        Ok(true) => {
            return Err(String::from(format!("Cannot insert duplicate word")));
        }
        Ok(false) => match insert_into(definition).values(new_def).execute(conn) {
            Ok(_) => return Ok(()),
            Err(e) => {
                return Err(String::from(format!("Cannot insert word: {}", e)));
            }
        },
        Err(e) => {
            return Err(String::from(format!("Unknow Error: {}", e)));
        }
    }
}

pub fn insert_meaning(conn: &mut PgConnection, new_meaning: NewMeaning) -> Result<i32, String> {
    use super::schema::meaning::dsl::*;
    match insert_into(meaning)
        .values(new_meaning.clone())
        .execute(conn)
    {
        Ok(_) => {
            let result = get_mean(conn, new_meaning.word.to_string()).ok().unwrap();
            return Ok(result[0].meaning_id);
        }
        Err(e) => return Err(e.to_string()),
    }
}

pub fn get_def(conn: &mut PgConnection, new_word: String) -> Result<Vec<Definition>, String> {
    use super::schema::definition::dsl::*;
    let defn: Result<Vec<Definition>, Error> = definition
        .filter(word.eq(new_word))
        .limit(1)
        .load::<Definition>(conn);

    match defn {
        Ok(t) => {
            return Ok(t);
        }
        Err(_) => return Err(String::from(format!("Cannot find word"))),
    }
}

pub fn get_mean(conn: &mut PgConnection, word_query: String) -> Result<Vec<Meaning>, String> {
    use super::schema::meaning::dsl::*;
    let def_mean: Result<Vec<Meaning>, Error> = meaning
        .filter(word.eq(word_query))
        .limit(1)
        .load::<Meaning>(conn);

    match def_mean {
        Ok(t) => return Ok(t),
        Err(_) => return Err(String::from(format!("Cannot find meaning"))),
    }
}

pub fn delete_word(new_word: String) -> Result<usize, String> {
    use super::schema::meaning::dsl::*;
    let mut conn = establish_connection();
    match diesel::delete(meaning.filter(word.eq(new_word))).execute(&mut conn) {
        Ok(t) => Ok(t),
        Err(_) => Err(String::from(format!("Word not found"))),
    }
}

pub fn get_result(
    conn: &mut PgConnection,
    query_word: String,
) -> Result<(Definition, Meaning), String> {
    let defin = get_def(conn, query_word.clone())?;

    if defin.len() == 0 {
        return Err("Cannot find definition".to_string());
    }
    let mean = get_mean(conn, query_word)?;

    return Ok((defin[0].clone(), mean[0].clone()));
}
