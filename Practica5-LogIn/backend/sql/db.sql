CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password BLOB NOT NULL,
    salt BLOB NOT NULL,
    verified BOOLEAN DEFAULT 0
);

-- Recovery tokens: only the SHA3-256 hash of the opaque token is stored,
-- and expires_at is a unix timestamp (token lives 1 hour).
CREATE TABLE IF NOT EXISTS restore_token (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token_hash TEXT NOT NULL UNIQUE,
    expires_at INTEGER NOT NULL,
    status TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
