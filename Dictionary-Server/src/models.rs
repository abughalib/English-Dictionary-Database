use super::schema::definition;

#[derive(QueryableByName, Queryable, Eq, PartialEq, Debug, Clone)]
#[table_name="definition"]
pub struct Definition{
  pub id: i32,
  pub word: String,
  pub meaning: Option<Vec<String>>,
  pub antonyms: Option<Vec<String>>,
  pub synonyms: Option<Vec<String>>
}

#[derive(Insertable)]
#[table_name="definition"] 
pub struct NewDefinition<'a>{
  pub word: &'a str,
  pub meaning: Option<Vec<&'a str>>,
  pub antonyms: Option<Vec<&'a str>>,
  pub synonyms: Option<Vec<&'a str>>,
}