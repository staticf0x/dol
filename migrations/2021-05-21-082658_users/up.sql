-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    username VARCHAR(64) NOT NULL
);
CREATE UNIQUE INDEX username_unique ON users(username);
