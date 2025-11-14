-- Drop indexes
DROP INDEX IF EXISTS idx_transaction_digests_checkpoint;
DROP INDEX IF EXISTS idx_smart_contract_objects_checkpoint;
DROP INDEX IF EXISTS idx_smart_contract_objects_type;
DROP INDEX IF EXISTS idx_smart_contract_objects_owner;
DROP INDEX IF EXISTS idx_datapod_events_checkpoint;
DROP INDEX IF EXISTS idx_datapod_events_seller;
DROP INDEX IF EXISTS idx_datapod_events_datapod_id;

-- Drop tables
DROP TABLE IF EXISTS transaction_digests;
DROP TABLE IF EXISTS smart_contract_objects;
DROP TABLE IF EXISTS datapod_events;
