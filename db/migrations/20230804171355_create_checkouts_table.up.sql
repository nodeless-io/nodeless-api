CREATE TABLE checkouts (
    uuid VARCHAR(255) UNIQUE PRIMARY KEY,
    user_uuid VARCHAR(255) references users(uuid) NOT NULL,
    amount BIGINT NOT NULL,
    status checkout_status NOT NULL,
    bitcoin_address VARCHAR(255) NOT NULL,
    payment_request TEXT NOT NULL,
    expiry_seconds BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    expired_at TIMESTAMP,
    deleted_at TIMESTAMP
);