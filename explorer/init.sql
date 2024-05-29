CREATE TABLE batches (
    merkle_root VARCHAR(255) PRIMARY KEY,
    qty INT NOT NULL,
    is_verified BOOLEAN NOT NULL
);

