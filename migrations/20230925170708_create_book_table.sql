CREATE TABLE bookShelf(
id uuid NOT NULL,
PRIMARY KEY (id),
name TEXT NOT NULL UNIQUE,
author TEXT NOT NULL,
finished BOOLEAN NOT NULL,
added_at timestamptz NOT NULL
);
