-- Add up migration script here
CREATE TABLE nodeless_addresses (
    uuid VARCHAR(255) UNIQUE PRIMARY KEY,
    user_uuid VARCHAR(255) references users(uuid) NOT NULL,
    handle VARCHAR(255) UNIQUE NOT NULL,
    npub VARCHAR(255) UNIQUE,
    price BIGINT NOT NULL DEFAULT 1000,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);