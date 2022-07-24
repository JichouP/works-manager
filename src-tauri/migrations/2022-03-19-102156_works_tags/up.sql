-- Your SQL goes here
CREATE TABLE works_tags
(
  work_id    INTEGER NOT NULL,
  tag_id     INTEGER NOT NULL,
  created_at TEXT    NOT NULL DEFAULT current_timestamp,
  updated_at TEXT    NOT NULL DEFAULT current_timestamp,
  PRIMARY KEY (work_id, tag_id),
  FOREIGN KEY (work_id) REFERENCES works (id),
  FOREIGN KEY (tag_id) REFERENCES tags (id)
);