-- Add migration script here
CREATE TABLE IF NOT EXISTS
    recipes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        author TEXT,
        name TEXT NOT NULL,
        description TEXT,
        difficulty TEXT,
        estimated_duration TEXT
    );

CREATE TABLE IF NOT EXISTS
    ingredients (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        recipe_id INTEGER NOT NULL,
        position INTEGER NOT NULL,
        description TEXT NOT NULL,
        FOREIGN KEY (recipe_id) REFERENCES recipes (id) ON DELETE CASCADE
    );

CREATE TABLE IF NOT EXISTS
    instructions (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        recipe_id INTEGER NOT NULL,
        position INTEGER NOT NULL,
        description TEXT NOT NULL,
        FOREIGN KEY (recipe_id) REFERENCES recipes (id) ON DELETE CASCADE
    );
