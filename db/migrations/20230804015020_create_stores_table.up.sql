-- Add up migration script here
CREATE TABLE stores (
    uuid VARCHAR(255) UNIQUE PRIMARY KEY,
    user_uuid VARCHAR(255) references users(uuid) NOT NULL,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);