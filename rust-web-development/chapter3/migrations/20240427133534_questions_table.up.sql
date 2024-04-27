-- Add up migration script here
CREATE TABLE IF NOT EXISTS questions (
  id serial PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  content TEXT NOT NULL,
  tags TEXT [],
  created_on TIMESTAP NOT NULL DEFAULT NOW()
);
