use super::schema::{definition, meaning};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(QueryableByName, Queryable, Eq, PartialEq, Debug, Clone)]
#[diesel(table_name = definition)]
pub struct Definition {
    pub word_id: i32,
    pub word: String,
    pub meaning_id: i32,
    pub antonyms: Vec<Option<String>>,
    pub synonyms: Vec<Option<String>>,
}

#[derive(Queryable, QueryableByName, Eq, PartialEq, Debug, Clone)]
#[diesel(table_name = meaning)]
pub struct Meaning {
    pub meaning_id: i32,
    pub word: String,
    pub def: Vec<Option<String>>,
    pub keywords: Vec<Option<String>>,
}

#[derive(Insertable)]
#[diesel(table_name = definition)]
pub struct NewDefinition<'a> {
    pub word: &'a str,
    pub meaning_id: &'a i32,
    pub antonyms: Vec<&'a str>,
    pub synonyms: Vec<&'a str>,
}

#[derive(Insertable, Clone)]
#[diesel(table_name = meaning)]
pub struct NewMeaning<'a> {
    pub word: &'a str,
    pub def: Vec<&'a str>,
    pub keywords: Vec<&'a str>,
}

#[derive(Deserialize, Serialize)]
pub struct QueryWord {
    pub word: String,
}

#[derive(Serialize, Debug)]
pub struct QueryDefinition {
    pub word: String,
    pub meaning: QueryMeaning,
    pub synonyms: Vec<Option<String>>,
    pub antonyms: Vec<Option<String>>,
}
#[derive(Serialize, Debug)]
pub struct QueryMeaning {
    pub def: Vec<Option<String>>,
    pub keywords: Vec<Option<String>>,
}

#[derive(Serialize, QueryableByName, Deserialize)]
#[diesel(table_name = meaning)]
pub struct SearchWord {
    pub word: String
}