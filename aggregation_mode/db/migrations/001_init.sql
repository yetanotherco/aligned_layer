CREATE TYPE task_status AS ENUM ('pending', 'verified');

CREATE TABLE tasks (
    task_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    status task_status DEFAULT 'pending'
);

CREATE TABLE proofs (
    proof_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    address CHAR(42), 
    proving_system_id INT,
    proof BYTEA,
    program_commitment BYTEA,
    merkle_path BYTEA,
    task_id UUID REFERENCES tasks(task_id)
);

CREATE TABLE payment_events (
    payment_event_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    address CHAR(42),
    amount BIGINT,
    started_at BIGINT,
    valid_until BIGINT,
    tx_hash CHAR(66) UNIQUE
);
