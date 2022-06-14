-- Your SQL goes here
CREATE INDEX CONCURRENTLY word ON meaning USING HASH(word);