
CREATE TABLE associations
(
  id         INTEGER NOT NULL,
  created_at TEXT    NOT NULL DEFAULT current_timestamp,
  updated_at TEXT    NOT NULL DEFAULT current_timestamp,
  name       TEXT    NOT NULL,
  PRIMARY KEY (id AUTOINCREMENT)
);

CREATE TABLE configs
(
  id         INTEGER NOT NULL,
  created_at TEXT    NOT NULL DEFAULT current_timestamp,
  updated_at TEXT    NOT NULL DEFAULT current_timestamp,
  base_path  TEXT    NOT NULL,
  PRIMARY KEY (id AUTOINCREMENT)
);

CREATE TABLE tags
(
  id         INTEGER NOT NULL,
  created_at TEXT    NOT NULL DEFAULT current_timestamp,
  updated_at TEXT    NOT NULL DEFAULT current_timestamp,
  name       TEXT    NOT NULL,
  PRIMARY KEY (id AUTOINCREMENT)
);

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
