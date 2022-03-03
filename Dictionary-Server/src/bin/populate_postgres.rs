use std::{fs::File, io::Read};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use dictionary_server::{database_op::{establish_connection, insert_definition, insert_meaning}, models::{NewDefinition, NewMeaning}};

#[derive(Debug, Serialize, Deserialize)]
pub struct WordJson{
  meaning: Meaning,
  antonyms: Vec<String>,
  synonyms: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Meaning{
  def: Vec<String>,
  keywords: Vec<String>,
}

pub fn load_json(){
  let mut words = File::open(
    "assets/dict_words.txt").ok().expect("Cannot find words file");

  let mut words_str = String::new();
  let mut word_char = 'B';

  words.read_to_string(&mut words_str)
  .ok().expect("Cannot parse words to string");

  let mut dict_file = File::open(
    "assets/DA_mod.json").expect("Failed to open First File");
  
  let mut dict_str = String::new();
  dict_file.read_to_string(&mut dict_str).ok()
  .expect("Failed to convert dict_file to dict_str && word");

  let mut dict_json: Value = serde_json::from_str(
  dict_str.as_str()).ok().expect("Failed to load json from str");

  for word in words_str.split_whitespace(){
    
    if word_char != word.chars().nth(0).expect(format!("No First Char of word: {}", word).as_str()){
      word_char = word.chars().nth(0).unwrap();
      let mut dict_file = File::open(
      format!("assets/D{}_mod.json", word_char)
      ).ok().unwrap();
      dict_str = String::new();
      dict_file.read_to_string(&mut dict_str).ok()
      .expect("Failed to convert dict_file to dict_str");
      dict_json = serde_json::from_str(
      dict_str.as_str()).ok()
      .expect(format!("Failed to convert dict_file to dict_json, Word: {}", word).as_str());
    }

    println!("{}", word);
    let meaning_id = insert_parsed_meaning(word.to_string().clone(), dict_json[word]["meaning"].clone());
    insert_parsed_definition(word.to_string().clone(), &meaning_id, dict_json[word].clone());

  }

}

fn insert_parsed_meaning<'a>(word: String, meaning: Value)->i32{
  let mut def: Vec<&str> = Vec::new();
  let mut keywords: Vec<&str> = Vec::new();

  match meaning["def"].as_array(){
    Some(t)=>{
      for i in t{
        def.push(i.as_str().unwrap());
      }
    },
    None=>{
      def = vec![""]
    }
  }

  match meaning["keywords"].as_array(){
    Some(t)=>{
      for i in t{
        keywords.push(i.as_str().unwrap());
      }
    },
    None=>{
      keywords = vec![""]
    }
  }

  let new_meaning = NewMeaning{
    word: word.as_str(),
    def,
    keywords
  };

  println!("DEF: {:?} \nKeywords: {:?}", new_meaning.def, new_meaning.keywords);

  let conn = establish_connection();
  match insert_meaning(&conn, new_meaning){
    Ok(index)=>{
      return index;
    },
    Err(e)=>{
      panic!("Cannot insert value: {:?}", e);
    }
  }
}

fn insert_parsed_definition<'a>(word: String, meaning_id: &i32, dict: Value){

  let mut synonyms: Vec<&str> = Vec::new();
  let mut antonyms: Vec<&str> = Vec::new();

   match dict["synonyms"].as_array(){
    Some(t)=>{
      for i in t{
        synonyms.push(i.as_str().unwrap());
      }
    },
    None=>{
      synonyms = vec![""]
    }
  }

  match dict["antonyms"].as_array(){
    Some(t)=>{
      for i in t{
        antonyms.push(i.as_str().unwrap());
      }
    },
    None=>{
      antonyms = vec![""]
    }
  }
  let conn = establish_connection();

  let new_def = NewDefinition{
    word: word.as_str(),
    meaning_id,
    antonyms,
    synonyms
  };

  println!("Antonyms: {:?}\nSynonyms: {:?}", new_def.antonyms, new_def.synonyms);

  insert_definition(&conn, new_def).ok().expect("Failed to insert word");
}

fn main(){
  load_json();
}