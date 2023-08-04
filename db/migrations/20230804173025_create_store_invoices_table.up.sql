-- Add up migration script here
CREATE TABLE store_invoices (
    uuid VARCHAR(255) UNIQUE PRIMARY KEY,
    store_uuid VARCHAR(255) references stores(uuid) NOT NULL,
    checkout_uuid VARCHAR(255) references checkouts(uuid) NOT NULL,
    metadata JSONB,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);