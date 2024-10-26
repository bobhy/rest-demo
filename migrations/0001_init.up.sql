-- initial "migration" -- create the basic schema
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR(250) NOT NULL,
    email VARCHAR(250),
    active BOOLEAN NOT NULL DEFAULT 0
);