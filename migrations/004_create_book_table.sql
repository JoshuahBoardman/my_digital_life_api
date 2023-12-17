CREATE TABLE books(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL UNIQUE,
    author TEXT NOT NULL,
    description TEXT NOT NULL,
    rating DECIMAL(3, 2) NOT NULL CHECK (rating >= 0 AND rating <= 5),
    review TEXT, 
    finished BOOLEAN NOT NULL,
    inserted_at TIMESTAMPTZ NOT NULL
);
