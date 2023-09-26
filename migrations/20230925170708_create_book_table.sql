CREATE TABLE subscriptions(
id uuid NOT NULL,
PRIMARY KEY (id),
name TEXT NOT NULL UNIQUE,
author TEXT NOT NULL,
added_at timestamptz NOT NULL,
finished BOOLEAN NOT NULL
);
