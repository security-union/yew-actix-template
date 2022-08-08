-- migrate:up
CREATE TABLE oauth_requests (
    state: VARCHAR(255),
);

CREATE TABLE users (
    email: VARCHAR(255) PRIMARY KEY,
    access_token: TEXT,
    refresh_token: TEXT
);

-- migrate:down
DROP TABLE oauth_requests;
DROP TABLE users;
