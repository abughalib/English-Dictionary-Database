use super::schema::definition;

#[derive(QueryableByName, Queryable)]
#[table_name="definition"]
pub struct Definition{
  pub id: i32,
  pub word: String,
  pub meaning: Option<Vec<Vec<String>>>,
  pub antonyms: Option<Vec<String>>,
  pub synonyms: Option<Vec<String>>,
}

#[derive(Insertable)]
#[table_name="definition"] 
pub struct NewDefinition{
  pub word: String,
  pub meaning: Option<Vec<Vec<String>>>,
  pub antonyms: Option<Vec<String>>,
  pub synonyms: Option<Vec<String>>,
}