-- Your SQL goes here
CREATE TABLE works
(
  id         INTEGER NOT NULL,
  created_at TEXT    NOT NULL DEFAULT current_timestamp,
  updated_at TEXT    NOT NULL DEFAULT current_timestamp,
  name       TEXT    NOT NULL,
  path       TEXT    NOT NULL,
  thumbnail  TEXT    NULL    ,
  rate       INTEGER NOT NULL DEFAULT 5,
  PRIMARY KEY (id AUTOINCREMENT)
);