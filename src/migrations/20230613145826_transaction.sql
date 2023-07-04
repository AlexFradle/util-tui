CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    amount REAL NOT NULL,
    details TEXT,
    date TEXT NOT NULL
);
