-- Your SQL goes here
CREATE TABLE questions (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
  content TEXT NOT NULL,
  yes_id INTEGER,
  no_id INTEGER,
  is_last INTEGER NOT NULL,
  FOREIGN KEY(yes_id) REFERENCES questions(id),
  FOREIGN KEY(no_id) REFERENCES questions(id)
);