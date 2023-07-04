CREATE TABLE IF NOT EXISTS episodes (
    id INTEGER PRIMARY KEY NOT NULL,
    movie_id INTEGER NOT NULL,
    series_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    rating REAL NOT NULL,
    date_watched TEXT,
    FOREIGN KEY(movie_id) REFERENCES movies(id),
    FOREIGN KEY(series_id) REFERENCES series(id)
);
