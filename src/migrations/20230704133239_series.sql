CREATE TABLE IF NOT EXISTS series (
    id INTEGER PRIMARY KEY NOT NULL,
    movie_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    rating REAL NOT NULL,
    FOREIGN KEY(movie_id) REFERENCES movies(id)
);
