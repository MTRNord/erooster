CREATE TABLE IF NOT EXISTS users (
    username TEXT NOT NULL PRIMARY KEY UNIQUE,
    hash TEXT
);
