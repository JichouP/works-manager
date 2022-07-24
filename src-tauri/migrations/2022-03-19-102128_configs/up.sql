-- Your SQL goes here
CREATE TABLE configs
(
  id         INTEGER NOT NULL,
  created_at TEXT    NOT NULL DEFAULT current_timestamp,
  updated_at TEXT    NOT NULL DEFAULT current_timestamp,
  base_path  TEXT    NOT NULL,
  PRIMARY KEY (id AUTOINCREMENT)
);
