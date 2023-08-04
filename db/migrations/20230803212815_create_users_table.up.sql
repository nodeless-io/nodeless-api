CREATE type checkout_status as enum (
    'new', 'pendingconfirmation', 'paid', 'overpaid', 'underpaid', 'expired'
);

CREATE TABLE users (
    uuid VARCHAR(255) UNIQUE PRIMARY KEY,
    email VARCHAR(255) UNIQUE,
    password VARCHAR(255),
    npub VARCHAR(255) UNIQUE,
    identifier VARCHAR(50) UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);
