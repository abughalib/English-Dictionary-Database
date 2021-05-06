use super::schema::{definition, meaning};

#[derive(QueryableByName, Queryable, Eq, PartialEq, Debug, Clone)]
#[table_name="definition"]
pub struct Definition{
  pub word_id: i32,
  pub word: String,
  pub meaning_id: i32,
  pub antonyms: Vec<String>,
  pub synonyms: Vec<String>,
}

#[derive(Queryable, QueryableByName, Eq, PartialEq, Debug, Clone)]
#[table_name="meaning"]
pub struct Meaning{
  pub meaning_id: i32,
  pub word: String,
  pub def: Vec<String>,
  pub keywords: Vec<String>
}

#[derive(Insertable)]
#[table_name="definition"] 
pub struct NewDefinition<'a>{
  pub word: &'a str,
  pub meaning_id: &'a i32,
  pub antonyms: Vec<&'a str>,
  pub synonyms: Vec<&'a str>,
}

#[derive(Insertable, Clone)]
#[table_name="meaning"]
pub struct NewMeaning<'a>{
  pub word: &'a str,
  pub def: Vec<&'a str>,
  pub keywords: Vec<&'a str>
}