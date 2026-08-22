-- Add up migration script here
CREATE TABLE IF NOT EXISTS project (
  path TEXT NOT NULL PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS commits (
  id TEXT NOT NULL PRIMARY KEY,
  branch TEXT NOT NULL,
  hash TEXT NOT NULL,
  model TEXT NOT NULL CHECK (model in ("fp32", "fp16", "int8")),
  message TEXT NOT NULL,
  project TEXT NOT NULL,
  complete BOOLEAN NOT NULL DEFAULT 0 CHECK (complete IN (0, 1)),
  FOREIGN KEY (project) REFERENCES project (path) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS file (
  path TEXT NOT NULL PRIMARY KEY,
  commit_id TEXT NOT NULL,
  FOREIGN KEY (commit_id) REFERENCES commits(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS code (
  chunk TEXT NOT NULL,
  hash TEXT NOT NUll,
  embedding_id TEXT NOT NULL,
  file_id   TEXT    NOT NULL,
  FOREIGN KEY (file_id) REFERENCES file(id) ON DELETE CASCADE
);
