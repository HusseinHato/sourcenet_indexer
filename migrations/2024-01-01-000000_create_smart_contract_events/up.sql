-- Create table for storing smart contract events from DataPod module
CREATE TABLE IF NOT EXISTS datapod_events (
    id BIGSERIAL PRIMARY KEY,
    event_type VARCHAR(255) NOT NULL,
    datapod_id VARCHAR(255) NOT NULL,
    seller VARCHAR(255) NOT NULL,
    title VARCHAR(1024),
    category VARCHAR(255),
    price_sui BIGINT,
    kiosk_id VARCHAR(255),
    old_price BIGINT,
    new_price BIGINT,
    transaction_digest VARCHAR(255) NOT NULL,
    checkpoint_sequence_number BIGINT NOT NULL,
    event_index BIGINT NOT NULL,
    timestamp BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(transaction_digest, event_index)
);

-- Create table for storing smart contract objects
CREATE TABLE IF NOT EXISTS smart_contract_objects (
    id BIGSERIAL PRIMARY KEY,
    object_id VARCHAR(255) NOT NULL UNIQUE,
    object_type VARCHAR(1024) NOT NULL,
    owner VARCHAR(255),
    version BIGINT NOT NULL,
    digest VARCHAR(255) NOT NULL,
    content_type VARCHAR(255),
    data JSONB,
    checkpoint_sequence_number BIGINT NOT NULL,
    transaction_digest VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create table for storing transaction digests for indexing
CREATE TABLE IF NOT EXISTS transaction_digests (
    id BIGSERIAL PRIMARY KEY,
    tx_digest VARCHAR(255) NOT NULL UNIQUE,
    checkpoint_sequence_number BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_datapod_events_datapod_id ON datapod_events(datapod_id);
CREATE INDEX IF NOT EXISTS idx_datapod_events_seller ON datapod_events(seller);
CREATE INDEX IF NOT EXISTS idx_datapod_events_checkpoint ON datapod_events(checkpoint_sequence_number);
CREATE INDEX IF NOT EXISTS idx_smart_contract_objects_owner ON smart_contract_objects(owner);
CREATE INDEX IF NOT EXISTS idx_smart_contract_objects_type ON smart_contract_objects(object_type);
CREATE INDEX IF NOT EXISTS idx_smart_contract_objects_checkpoint ON smart_contract_objects(checkpoint_sequence_number);
CREATE INDEX IF NOT EXISTS idx_transaction_digests_checkpoint ON transaction_digests(checkpoint_sequence_number);
