-- Add up migration script here
CREATE TABLE IF NOT EXISTS project (
    path TEXT PRIMARY KEY,
    name TEXT NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS commits (
    hash TEXT PRIMARY KEY,
    branch TEXT NOT NULL,
    model TEXT NOT NULL CHECK (model IN ('fp32', 'fp16', 'int8')),
    message TEXT NOT NULL,
    complete BOOLEAN NOT NULL DEFAULT 0 CHECK (complete IN (0, 1)),
    project TEXT NOT NULL REFERENCES project(path) ON DELETE CASCADE
) STRICT;

CREATE TABLE IF NOT EXISTS file (
    path TEXT PRIMARY KEY,
    commit_id TEXT NOT NULL REFERENCES commits(id) ON DELETE CASCADE
 ) STRICT;

CREATE TABLE IF NOT EXISTS file_commits (
    file_id TEXT    NOT NULL REFERENCES file(path) ON DELETE CASCADE,
    commit_hash   TEXT NOT NULL REFERENCES commits(hash) ON DELETE CASCADE,
    PRIMARY KEY (file_id, commit_id)
) STRICT;
 
CREATE TABLE IF NOT EXISTS code (
    chunk TEXT    NOT NULL,
    hash TEXT NOT NUll,
    symbol  TEXT    NOT NULL,
    vector  BLOB NOT NULL,
    file_id   TEXT    NOT NULL REFERENCES file(id) ON DELETE CASCADE
) STRICT;