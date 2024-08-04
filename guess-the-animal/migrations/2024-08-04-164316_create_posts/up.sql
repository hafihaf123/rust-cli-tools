-- Your SQL goes here
CREATE TABLE questions (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  question TEXT NOT NULL,
  yes_id INTEGER,
  no_id INTEGER,
  FOREIGN KEY(yes_id) REFERENCES questions(id),
  FOREIGN KEY(no_id) REFERENCES questions(id)
);

CREATE TABLE animals (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL
);