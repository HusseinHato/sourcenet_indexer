use std::sync::Arc;

use anyhow::{Result};
use async_trait::async_trait;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;

use sui_indexer_alt_framework::{
    pipeline::sequential::Handler,
    pipeline::Processor,
    postgres::{Connection, Db},
};
use sui_types::full_checkpoint_content::CheckpointData;

use crate::models::{StoredDataPodEvent, StoredSmartContractObject, StoredTransactionDigest};
use crate::schema;

/// Handler for processing transaction digests from checkpoints
pub struct TransactionDigestHandler;

#[async_trait]
impl Processor for TransactionDigestHandler {
    const NAME: &'static str = "transaction_digest_handler";
    type Value = StoredTransactionDigest;

    async fn process(&self, checkpoint: &Arc<CheckpointData>) -> Result<Vec<Self::Value>> {
        let checkpoint_seq = checkpoint.checkpoint_summary.sequence_number as i64;
        let digests = checkpoint
            .transactions
            .iter()
            .map(|tx| StoredTransactionDigest {
                tx_digest: tx.transaction.digest().to_string(),
                checkpoint_sequence_number: checkpoint_seq,
            })
            .collect();
        Ok(digests)
    }
}

#[async_trait]
impl Handler for TransactionDigestHandler {
    type Store = Db;
    type Batch = Vec<Self::Value>;

    fn batch(batch: &mut Self::Batch, values: Vec<Self::Value>) {
        batch.extend(values);
    }

    async fn commit<'a>(
        batch: &Self::Batch,
        conn: &mut Connection<'a>,
    ) -> Result<usize> {
        use schema::transaction_digests::dsl::*;
        diesel::insert_into(transaction_digests)
            .values(batch)
            .on_conflict(tx_digest)
            .do_nothing()
            .execute(conn)
            .await
            .map_err(Into::into)
    }
}

/// Handler for processing DataPod events from smart contracts
#[allow(dead_code)]
pub struct DataPodEventHandler;

#[async_trait]
impl Processor for DataPodEventHandler {
    const NAME: &'static str = "datapod_event_handler";
    type Value = StoredDataPodEvent;

    async fn process(&self, checkpoint: &Arc<CheckpointData>) -> Result<Vec<Self::Value>> {
        let checkpoint_seq = checkpoint.checkpoint_summary.sequence_number as i64;
        let timestamp_ms = checkpoint.checkpoint_summary.timestamp_ms as i64;
        let mut events = Vec::new();

        // Read smart contract address from environment
        let _smart_contract_address = std::env::var("SMART_CONTRACT_ADDRESS")
            .unwrap_or_else(|_| "0x0000000000000000000000000000000000000000000000000000000000000000".to_string());

        for tx in checkpoint.transactions.iter() {
            let tx_digest_str = tx.transaction.digest().to_string();

            for (event_idx, event) in tx.events.iter().enumerate() {
                // Event data is in the `data` field (BCS encoded Move struct)
                // To extract specific fields, you need to deserialize based on your Move struct
                // For now, we store all events with placeholder values
                
                let stored_event = StoredDataPodEvent {
                    event_type: "datapod_event".to_string(),
                    datapod_id: String::new(), // TODO: Extract from event.data by deserializing BCS
                    seller: String::new(),     // TODO: Extract from event.data by deserializing BCS
                    title: None,
                    category: None,
                    price_sui: None,
                    kiosk_id: None,
                    old_price: None,
                    new_price: None,
                    transaction_digest: tx_digest_str.clone(),
                    checkpoint_sequence_number: checkpoint_seq,
                    event_index: event_idx as i64,
                    timestamp: timestamp_ms,
                };
                events.push(stored_event);
            }
        }

        Ok(events)
    }
}

#[async_trait]
impl Handler for DataPodEventHandler {
    type Store = Db;
    type Batch = Vec<Self::Value>;

    fn batch(batch: &mut Self::Batch, values: Vec<Self::Value>) {
        batch.extend(values);
    }

    async fn commit<'a>(
        batch: &Self::Batch,
        conn: &mut Connection<'a>,
    ) -> Result<usize> {
        use schema::datapod_events::dsl::*;
        diesel::insert_into(datapod_events)
            .values(batch)
            .on_conflict((transaction_digest, event_index))
            .do_nothing()
            .execute(conn)
            .await
            .map_err(Into::into)
    }
}

/// Handler for processing smart contract objects
#[allow(dead_code)]
pub struct SmartContractObjectHandler;

#[async_trait]
impl Processor for SmartContractObjectHandler {
    const NAME: &'static str = "smart_contract_object_handler";
    type Value = StoredSmartContractObject;

    async fn process(&self, checkpoint: &Arc<CheckpointData>) -> Result<Vec<Self::Value>> {
        let checkpoint_seq = checkpoint.checkpoint_summary.sequence_number as i64;
        let mut objects = Vec::new();

        for tx in checkpoint.transactions.iter() {
            let tx_digest_str = tx.transaction.digest().to_string();

            for (obj_ref, owner, _write_kind) in tx.effects.all_changed_objects() {
                let stored_object = StoredSmartContractObject {
                    object_id: obj_ref.0.to_string(),
                    object_type: String::new(),
                    owner: Some(owner.to_string()),
                    version: obj_ref.1.value() as i64,
                    digest: obj_ref.2.to_string(),
                    content_type: None,
                    checkpoint_sequence_number: checkpoint_seq,
                    transaction_digest: tx_digest_str.clone(),
                };
                objects.push(stored_object);
            }
        }

        Ok(objects)
    }
}

#[async_trait]
impl Handler for SmartContractObjectHandler {
    type Store = Db;
    type Batch = Vec<Self::Value>;

    fn batch(batch: &mut Self::Batch, values: Vec<Self::Value>) {
        batch.extend(values);
    }

    async fn commit<'a>(
        batch: &Self::Batch,
        conn: &mut Connection<'a>,
    ) -> Result<usize> {
        use schema::smart_contract_objects::dsl::*;
        diesel::insert_into(smart_contract_objects)
            .values(batch)
            .on_conflict(object_id)
            .do_update()
            .set((
                version.eq(diesel::dsl::sql("excluded.version")), // Use excluded to refer to the new value
                digest.eq(diesel::dsl::sql("excluded.digest")),
            ))
            .execute(conn)
            .await
            .map_err(Into::into)
    }
}
