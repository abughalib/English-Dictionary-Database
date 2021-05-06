-- Your SQL goes here
CREATE TABLE IF NOT EXISTS meaning(
  id SERIAL PRIMARY KEY,
  def TEXT[] NOT NULL,
  keywords TEXT[] NOT NULL
);