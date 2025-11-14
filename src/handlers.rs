use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use sui_indexer_alt_framework::pipeline::Processor;
use sui_types::full_checkpoint_content::Checkpoint;
use diesel_async::RunQueryDsl;
use sui_indexer_alt_framework::{
    postgres::{Connection, Db},
    pipeline::sequential::Handler,
};

use crate::models::{StoredTransactionDigest, StoredDataPodEvent, StoredSmartContractObject};
use crate::schema::transaction_digests::dsl::*;
use crate::schema::datapod_events;
use crate::schema::smart_contract_objects;

/// Handler for processing transaction digests from checkpoints
pub struct TransactionDigestHandler;

impl Processor for TransactionDigestHandler {
    const NAME: &'static str = "transaction_digest_handler";
    type Value = StoredTransactionDigest;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let checkpoint_seq = checkpoint.summary.sequence_number as i64;
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

    fn batch(&self, batch: &mut Self::Batch, values: std::vec::IntoIter<Self::Value>) {
        batch.extend(values);
    }

    async fn commit<'a>(
        &self,
        batch: &Self::Batch,
        conn: &mut Connection<'a>,
    ) -> Result<usize> {
        let inserted = diesel::insert_into(transaction_digests)
            .values(batch)
            .on_conflict(tx_digest)
            .do_nothing()
            .execute(conn)
            .await?;
        Ok(inserted)
    }
}

/// Handler for processing DataPod events from smart contracts
pub struct DataPodEventHandler;

impl Processor for DataPodEventHandler {
    const NAME: &'static str = "datapod_event_handler";
    type Value = StoredDataPodEvent;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let checkpoint_seq = checkpoint.summary.sequence_number as i64;
        let mut events = Vec::new();

        // Process events from all transactions in the checkpoint
        for (tx_idx, tx) in checkpoint.transactions.iter().enumerate() {
            let tx_digest = tx.transaction.digest().to_string();
            let timestamp = checkpoint.summary.timestamp_ms as i64;

            // Process events from this transaction
            for (event_idx, event) in tx.events.iter().enumerate() {
                // Check if this is a DataPod event from the sourcenet::datapod module
                if event.type_.address.to_string() == "0x0" 
                    && event.type_.module.as_str() == "datapod"
                {
                    // Parse event based on type
                    let event_type = event.type_.name.as_str();
                    
                    // Extract common fields from event data
                    // Note: Event data parsing depends on the specific event structure
                    // This is a simplified version - adjust based on actual event format
                    
                    let stored_event = StoredDataPodEvent {
                        event_type: event_type.to_string(),
                        datapod_id: String::new(), // Extract from event data
                        seller: String::new(),      // Extract from event data
                        title: None,
                        category: None,
                        price_sui: None,
                        kiosk_id: None,
                        old_price: None,
                        new_price: None,
                        transaction_digest: tx_digest.clone(),
                        checkpoint_sequence_number: checkpoint_seq,
                        event_index: event_idx as i64,
                        timestamp,
                    };
                    
                    events.push(stored_event);
                }
            }
        }

        Ok(events)
    }
}

#[async_trait]
impl Handler for DataPodEventHandler {
    type Store = Db;
    type Batch = Vec<Self::Value>;

    fn batch(&self, batch: &mut Self::Batch, values: std::vec::IntoIter<Self::Value>) {
        batch.extend(values);
    }

    async fn commit<'a>(
        &self,
        batch: &Self::Batch,
        conn: &mut Connection<'a>,
    ) -> Result<usize> {
        let inserted = diesel::insert_into(datapod_events)
            .values(batch)
            .on_conflict((datapod_events::transaction_digest, datapod_events::event_index))
            .do_nothing()
            .execute(conn)
            .await?;
        Ok(inserted)
    }
}

/// Handler for processing smart contract objects
pub struct SmartContractObjectHandler;

impl Processor for SmartContractObjectHandler {
    const NAME: &'static str = "smart_contract_object_handler";
    type Value = StoredSmartContractObject;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let checkpoint_seq = checkpoint.summary.sequence_number as i64;
        let mut objects = Vec::new();

        // Process object changes from all transactions
        for tx in checkpoint.transactions.iter() {
            let tx_digest = tx.transaction.digest().to_string();

            // Process object changes
            for obj_change in tx.output_objects.iter() {
                // Extract object information
                // This is a simplified version - adjust based on actual object structure
                
                let stored_object = StoredSmartContractObject {
                    object_id: String::new(),      // Extract from object
                    object_type: String::new(),    // Extract from object
                    owner: None,
                    version: 0,
                    digest: String::new(),
                    content_type: None,
                    data: None,
                    checkpoint_sequence_number: checkpoint_seq,
                    transaction_digest: tx_digest.clone(),
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

    fn batch(&self, batch: &mut Self::Batch, values: std::vec::IntoIter<Self::Value>) {
        batch.extend(values);
    }

    async fn commit<'a>(
        &self,
        batch: &Self::Batch,
        conn: &mut Connection<'a>,
    ) -> Result<usize> {
        let inserted = diesel::insert_into(smart_contract_objects)
            .values(batch)
            .on_conflict(smart_contract_objects::object_id)
            .do_update()
            .set((
                smart_contract_objects::version.eq(smart_contract_objects::version + 1),
                smart_contract_objects::digest.eq(smart_contract_objects::digest),
            ))
            .execute(conn)
            .await?;
        Ok(inserted)
    }
}
