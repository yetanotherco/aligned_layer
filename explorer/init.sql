CREATE TABLE batches (
    merkle_root VARCHAR(255) PRIMARY KEY,
    amount_of_proofs INT NOT NULL,
    is_verified BOOLEAN NOT NULL,
    -- new params:
    submition_block_number INT NOT NULL,
    submition_transaction_hash VARCHAR(255) NOT NULL,
    submition_timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    response_block_number INT,
    response_transaction_hash VARCHAR(255),
    response_timestamp TIMESTAMP,
    data_pointer VARCHAR(255),
);
