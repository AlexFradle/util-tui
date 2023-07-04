CREATE TABLE IF NOT EXISTS movies (
    id INTEGER PRIMARY KEY NOT NULL,
    imdb_id TEXT NOT NULL,
    name TEXT NOT NULL,
    rating REAL NOT NULL,
    date_watched TEXT
);
