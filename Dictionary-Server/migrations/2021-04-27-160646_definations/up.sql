-- Your SQL goes here
CREATE TABLE IF NOT EXISTS meaning(
  meaning_id SERIAL PRIMARY KEY,
  word TEXT UNIQUE NOT NULL,
  def TEXT[] NOT NULL,
  keywords TEXT[] NOT NULL
);
