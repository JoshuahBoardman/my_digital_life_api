CREATE TABLE blog_posts(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    author_id UUID REFERENCES users NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    last_updated TIMESTAMPTZ NOT NULL,
    inserted_at TIMESTAMPTZ NOT NULL
);
