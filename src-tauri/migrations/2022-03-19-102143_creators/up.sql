-- Your SQL goes here
CREATE TABLE creators
(
  id             INTEGER NOT NULL,
  created_at     TEXT    NOT NULL DEFAULT current_timestamp,
  updated_at     TEXT    NOT NULL DEFAULT current_timestamp,
  name           TEXT    NOT NULL,
  association_id INTEGER NULL    ,
  PRIMARY KEY (id AUTOINCREMENT),
  FOREIGN KEY (association_id) REFERENCES associations (id)
);