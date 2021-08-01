CREATE TABLE users (
    id       INTEGER NOT NULL PRIMARY KEY,
    username TEXT    NOT NULL UNIQUE,
    password TEXT    NOT NULL
);

CREATE TABLE apps (
    id       INTEGER NOT NULL PRIMARY KEY,
    user_id  INTEGER NOT NULL REFERENCES users(id),
    name     TEXT    NOT NULL,
    username TEXT    NOT NULL,
    password TEXT    NOT NULL,
    UNIQUE (user_id, name)
);

CREATE TABLE versions (
    id     INTEGER NOT NULL PRIMARY KEY,
    app_id INTEGER NOT NULL REFERENCES apps(id),
    name   TEXT    NOT NULL,
    code   INTEGER NOT NULL,
    UNIQUE (app_id, name, code)
);

CREATE TABLE reports (
    id         INTEGER NOT NULL PRIMARY KEY,
    version_id INTEGER NOT NULL REFERENCES versions(id),
    report_id  TEXT    NOT NULL,
    crash_date TEXT    NOT NULL
);

INSERT INTO users (username, password) VALUES ('admin', 'admin');
INSERT INTO apps (user_id, name, username, password) VALUES (1, 'Test', 'test', 'test');
INSERT INTO versions (app_id, name, code) VALUES (1, '1.0.0', 1);
