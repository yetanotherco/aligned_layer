CREATE TYPE task_status AS ENUM ('pending', 'processing', 'verified');

CREATE TABLE tasks (
    task_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    address CHAR(42), 
    proving_system_id INT,
    proof BYTEA,
    program_commitment BYTEA,
    merkle_path BYTEA,
    status task_status DEFAULT 'pending',
    nonce BIGINT NOT NULL,
    inserted_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE payment_events (
    payment_event_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    address CHAR(42),
    amount BIGINT,
    started_at BIGINT,
    valid_until BIGINT,
    tx_hash CHAR(66) UNIQUE,
    inserted_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
