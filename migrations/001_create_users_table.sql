CREATE EXTENSION IF NOT EXISTS citext;
CREATE DOMAIN email AS citext
  CHECK ( value ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

CREATE TABLE users(
    id UUID NOT NULL,
    PRIMARY KEY (id),
    user_name TEXT NOT NULL, 
    email EMAIL NOT NULL,
    inserted_at TIMESTAMPTZ NOT NULL,
    last_updated TIMESTAMPTZ NOT NULL
);
