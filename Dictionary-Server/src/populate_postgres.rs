use std::{fs::File, io::Read};

use serde::{Serialize, Deserialize};

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

  words.read_to_string(&mut words_str)
  .ok().expect("Cannot parse words to string");

  let mut i: usize = 0;

  for word in words_str.split_whitespace(){
    print!("{}\n", word);
    if i == 10{
      break;
    }
    i+=1;
  }

}