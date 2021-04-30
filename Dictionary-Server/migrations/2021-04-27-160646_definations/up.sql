-- Your SQL goes here
CREATE TABLE IF NOT EXISTS definition(
  id SERIAL PRIMARY KEY, 
  word TEXT NOT NULL,
  meaning TEXT[],
  antonyms TEXT[],
  synonyms TEXT[]
);