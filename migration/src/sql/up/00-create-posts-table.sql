CREATE TABLE IF NOT EXISTS posts(
    id          SERIAL PRIMARY KEY,
    title       VARCHAR (255) NOT NULL,
    text        TEXT NOT NULL
);
