CREATE TYPE proof_status AS ENUM ('pending', 'processing', 'verified');

CREATE TABLE proofs (
    proof_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    address CHAR(42), 
    proving_system_id INT,
    proof BYTEA,
    program_commitment BYTEA,
    merkle_path BYTEA,
    status proof_status DEFAULT 'pending'
);

CREATE TABLE payment_events (
    payment_event_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    address CHAR(42),
    amount BIGINT,
    started_at BIGINT,
    valid_until BIGINT,
    tx_hash CHAR(66) UNIQUE
);
