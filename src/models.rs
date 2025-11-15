use diesel::prelude::*;
use sui_indexer_alt_framework::FieldCount;
use crate::schema::{datapod_events, smart_contract_objects, transaction_digests};

/// Represents a DataPod event from the smart contract
#[derive(Insertable, Debug, Clone, FieldCount)]
#[diesel(table_name = datapod_events)]
pub struct StoredDataPodEvent {
    pub event_type: String,
    pub datapod_id: String,
    pub seller: String,
    pub title: Option<String>,
    pub category: Option<String>,
    pub price_sui: Option<i64>,
    pub kiosk_id: Option<String>,
    pub old_price: Option<i64>,
    pub new_price: Option<i64>,
    pub transaction_digest: String,
    pub checkpoint_sequence_number: i64,
    pub event_index: i64,
    pub timestamp: i64,
}

/// Represents a smart contract object stored on-chain
#[derive(Insertable, Debug, Clone, FieldCount)]
#[diesel(table_name = smart_contract_objects)]
pub struct StoredSmartContractObject {
    pub object_id: String,
    pub object_type: String,
    pub owner: Option<String>,
    pub version: i64,
    pub digest: String,
    pub content_type: Option<String>,
    pub checkpoint_sequence_number: i64,
    pub transaction_digest: String,
}

/// Represents a transaction digest for indexing
#[derive(Insertable, Debug, Clone, FieldCount)]
#[diesel(table_name = transaction_digests)]
pub struct StoredTransactionDigest {
    pub tx_digest: String,
    pub checkpoint_sequence_number: i64,
}
