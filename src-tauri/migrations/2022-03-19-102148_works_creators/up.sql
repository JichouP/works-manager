-- Your SQL goes here
CREATE TABLE works_creators
(
  work_id    INTEGER NOT NULL,
  creator_id INTEGER NOT NULL,
  created_at TEXT    NOT NULL DEFAULT current_timestamp,
  updated_at TEXT    NOT NULL DEFAULT current_timestamp,
  PRIMARY KEY (work_id, creator_id),
  FOREIGN KEY (work_id) REFERENCES works (id),
  FOREIGN KEY (creator_id) REFERENCES creators (id)
);