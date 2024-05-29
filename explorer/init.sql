CREATE TABLE batches (
    merkle_root VARCHAR(255) PRIMARY KEY,
    amount_of_proofs INT NOT NULL,
    is_verified BOOLEAN NOT NULL
);

