CREATE TABLE verification_codes(
    id UUID NOT NULL,
    PRIMARY KEY (id),
    code TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMP NOT NULL,
    user_id UUID REFERENCES users Not NULL,
    inserted_at TIMESTAMPTZ NOT NULL
);
