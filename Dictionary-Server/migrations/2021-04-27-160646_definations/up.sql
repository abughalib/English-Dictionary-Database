-- Your SQL goes here
CREATE TABLE IF NOT EXISTS definition(
  word_id SERIAL PRIMARY KEY, 
  word TEXT NOT NULL,
  meaning_id INT NOT NULL,
  antonyms TEXT[] NOT NULL,
  synonyms TEXT[] NOT NULL,
  CONSTRAINT fk_constraint
    FOREIGN KEY(meaning_id)
      REFERENCES meaning(id)
);