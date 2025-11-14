# SourceNet Sui Blockchain Indexer

A custom indexer for Sui blockchain smart contracts, specifically designed to index DataPod and related smart contracts from the SourceNet data marketplace.

## Overview

This indexer:
- Connects to Sui Testnet or Mainnet checkpoint stores
- Processes blockchain checkpoints sequentially
- Extracts transaction data and smart contract events
- Stores indexed data in a local PostgreSQL database
- Implements efficient batching for optimal performance

## Prerequisites

- **Rust**: 1.70+ (install from https://rustup.rs/)
- **PostgreSQL**: 12+ (install from https://www.postgresql.org/download/)
- **Cargo**: Comes with Rust

## Project Structure

```
sourcenet-indexer/
├── src/
│   ├── main.rs              # Entry point and indexer setup
│   ├── models.rs            # Data structures for database records
│   ├── handlers.rs          # Processor and Handler implementations
│   └── schema.rs            # Diesel ORM schema definitions
├── migrations/              # Database migrations
│   └── 2024-01-01-000000_create_smart_contract_events/
│       ├── up.sql          # Create tables and indexes
│       └── down.sql        # Drop tables and indexes
├── smart_contracts/        # Sui Move smart contracts
│   ├── datapod.move
│   ├── escrow.move
│   └── purchase.move
├── Cargo.toml             # Project dependencies
├── .env.example           # Environment variables template
└── README.md              # This file
```

## Setup Instructions

### Step 1: Create PostgreSQL Database

```bash
# Create the database
createdb sui_indexer

# Verify connection
psql sui_indexer -c "\conninfo"
```

If you get a role error, create the user first:
```bash
sudo -u postgres createuser --superuser username
```

### Step 2: Configure Environment

Copy the example environment file and update with your database URL:

```bash
cp .env.example .env
```

Edit `.env` and set your PostgreSQL connection string:
```
DATABASE_URL=postgres://username@localhost:5432/sui_indexer
```

### Step 3: Build the Project

```bash
cargo build --release
```

This will:
- Download and compile all dependencies
- Compile the indexer binary
- Prepare database migrations

### Step 4: Run the Indexer

```bash
# Run with default settings (Testnet)
cargo run --release

# Or run the compiled binary directly
./target/release/sourcenet-indexer
```

#### Command-line Options

The indexer supports various command-line arguments:

```bash
# Start from a specific checkpoint
cargo run --release -- --first-checkpoint 1000

# Use a custom remote store URL
cargo run --release -- --remote-store-url https://checkpoints.mainnet.sui.io

# Specify checkpoint range
cargo run --release -- --first-checkpoint 1000 --last-checkpoint 2000
```

For all available options:
```bash
cargo run --release -- --help
```

## Database Schema

### Tables

#### `transaction_digests`
Stores transaction hashes from processed checkpoints.
- `id`: Primary key
- `tx_digest`: Transaction digest (unique)
- `checkpoint_sequence_number`: Checkpoint sequence
- `created_at`: Timestamp

#### `datapod_events`
Stores events emitted by the DataPod smart contract module.
- `id`: Primary key
- `event_type`: Type of event (DataPodCreated, DataPodPublished, etc.)
- `datapod_id`: DataPod identifier
- `seller`: Seller address
- `title`: DataPod title
- `category`: DataPod category
- `price_sui`: Price in SUI tokens
- `transaction_digest`: Associated transaction
- `checkpoint_sequence_number`: Checkpoint sequence
- `timestamp`: Event timestamp

#### `smart_contract_objects`
Stores smart contract objects and their state changes.
- `id`: Primary key
- `object_id`: Unique object identifier
- `object_type`: Type of the object
- `owner`: Current owner address
- `version`: Object version
- `digest`: Object digest
- `data`: JSON data payload
- `checkpoint_sequence_number`: Checkpoint sequence
- `transaction_digest`: Associated transaction

## Architecture

### Processor Pattern

The indexer uses the **Processor** trait to extract data from checkpoints:

```rust
impl Processor for TransactionDigestHandler {
    const NAME: &'static str = "transaction_digest_handler";
    type Value = StoredTransactionDigest;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        // Transform checkpoint data into your custom data structure
    }
}
```

### Handler Pattern

The **Handler** trait commits processed data to the database:

```rust
impl Handler for TransactionDigestHandler {
    type Store = Db;
    type Batch = Vec<Self::Value>;

    async fn commit<'a>(
        &self,
        batch: &Self::Batch,
        conn: &mut Connection<'a>,
    ) -> Result<usize> {
        // Insert batch into database
    }
}
```

### Sequential Pipeline

The indexer processes checkpoints in order with smart batching:
1. `process()` extracts data from each checkpoint
2. `batch()` accumulates values from multiple checkpoints
3. `commit()` writes the batch when limits are reached

This ensures consistency and optimal database performance.

## Extending the Indexer

### Adding a New Event Handler

1. Create a new `Processor` implementation in `handlers.rs`:

```rust
pub struct MyEventHandler;

impl Processor for MyEventHandler {
    const NAME: &'static str = "my_event_handler";
    type Value = StoredMyEvent;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        // Your processing logic
    }
}
```

2. Implement the `Handler` trait for database commits

3. Register in `main.rs`:

```rust
cluster.sequential_pipeline(
    MyEventHandler,
    SequentialConfig::default(),
).await?;
```

### Modifying the Database Schema

1. Create a new migration:

```bash
diesel migration generate add_new_column
```

2. Edit the migration files in `migrations/`

3. Run the indexer - migrations apply automatically on startup

## Monitoring and Debugging

### Check Indexer Progress

```bash
# Query the latest checkpoint processed
psql sui_indexer -c "SELECT MAX(checkpoint_sequence_number) FROM transaction_digests;"

# Count events by type
psql sui_indexer -c "SELECT event_type, COUNT(*) FROM datapod_events GROUP BY event_type;"
```

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run --release
```

## Performance Tuning

The indexer uses sequential processing with configurable batch sizes:

```rust
SequentialConfig {
    max_batch_checkpoints: 100,  // Checkpoints per batch
    checkpoint_lag: 10,           // Lag behind latest checkpoint
}
```

Adjust these values based on:
- Available memory
- PostgreSQL connection pool size
- Network latency to checkpoint store

## Troubleshooting

### Database Connection Error

```
DATABASE_URL must be set in the environment
```

**Solution**: Ensure `.env` file exists and contains a valid `DATABASE_URL`

### Migration Failure

**Solution**: Check PostgreSQL is running and accessible:
```bash
psql $DATABASE_URL -c "SELECT 'Connected';"
```

### Out of Memory

**Solution**: Reduce batch size in `SequentialConfig` or increase system memory

### Checkpoint Store Timeout

**Solution**: Use a different checkpoint store URL or increase timeout values

## References

- [Sui Documentation - Custom Indexer](https://docs.sui.io/guides/developer/advanced/custom-indexer)
- [Sui Indexer Framework](https://github.com/MystenLabs/sui/tree/testnet/crates/sui-indexer-alt-framework)
- [Diesel ORM Documentation](https://diesel.rs/)

## License

This project is part of the SourceNet ecosystem.

## Support

For issues or questions:
1. Check the [Sui Documentation](https://docs.sui.io/)
2. Review the [Sui GitHub Issues](https://github.com/MystenLabs/sui/issues)
3. Consult the [Diesel Documentation](https://diesel.rs/guides/)
