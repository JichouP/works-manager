-- Your SQL goes here
CREATE TABLE tags
(
  id         INTEGER NOT NULL,
  created_at TEXT    NOT NULL DEFAULT current_timestamp,
  updated_at TEXT    NOT NULL DEFAULT current_timestamp,
  name       TEXT    NOT NULL,
  PRIMARY KEY (id AUTOINCREMENT)
);