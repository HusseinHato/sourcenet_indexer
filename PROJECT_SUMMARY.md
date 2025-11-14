# Sui Smart Contract Indexer - Project Summary

## What Has Been Set Up

A complete Rust-based indexer for Sui blockchain smart contracts, specifically designed for the SourceNet data marketplace. The indexer follows the official [Sui Custom Indexer documentation](https://docs.sui.io/guides/developer/advanced/custom-indexer).

## Project Structure

```
sourcenet-indexer/
├── src/
│   ├── main.rs              # Entry point - sets up IndexerCluster
│   ├── models.rs            # Data structures (StoredTransactionDigest, StoredDataPodEvent, etc.)
│   ├── handlers.rs          # Processor and Handler trait implementations
│   └── schema.rs            # Diesel ORM schema definitions
├── migrations/
│   └── 2024-01-01-000000_create_smart_contract_events/
│       ├── up.sql           # Creates 3 tables with indexes
│       └── down.sql         # Rollback migration
├── smart_contracts/         # Sui Move smart contracts
│   ├── datapod.move
│   ├── escrow.move
│   └── purchase.move
├── Cargo.toml              # Project dependencies
├── diesel.toml             # Diesel configuration
├── .env.example            # Environment variables template
├── .gitignore              # Git ignore rules
├── README.md               # Full documentation
├── SETUP.md                # Detailed setup guide
├── QUICKSTART.md           # 5-minute quick start
└── PROJECT_SUMMARY.md      # This file
```

## Key Dependencies

### Core Framework
- **sui-indexer-alt-framework**: Official Sui indexer framework from testnet branch
- **sui-types**: Sui type definitions and checkpoint structures

### Database
- **diesel**: Type-safe ORM for PostgreSQL
- **diesel-async**: Async support for Diesel
- **diesel_migrations**: Automatic migration management

### Runtime & Utilities
- **tokio**: Async runtime with full features
- **anyhow**: Error handling
- **async-trait**: Async trait support
- **clap**: Command-line argument parsing
- **dotenvy**: Environment variable loading
- **url**: URL parsing

## Database Schema

### Tables Created

1. **transaction_digests**
   - Stores transaction hashes from processed checkpoints
   - Unique constraint on `tx_digest`
   - Indexed by `checkpoint_sequence_number`

2. **datapod_events**
   - Stores events emitted by DataPod smart contract
   - Event types: DataPodCreated, DataPodPublished, DataPodDelisted, DataPodPriceUpdated
   - Indexed by: datapod_id, seller, checkpoint_sequence_number

3. **smart_contract_objects**
   - Stores smart contract object states
   - Tracks object versions and ownership changes
   - Indexed by: owner, object_type, checkpoint_sequence_number

## Architecture

### Processing Pipeline

```
Sui Checkpoint Store (Testnet/Mainnet)
            ↓
    IndexerCluster
            ↓
    Sequential Pipeline
            ↓
    Processor (extract data)
            ↓
    Handler (batch & commit)
            ↓
    PostgreSQL Database
```

### Key Components

**Processor Trait**
- Transforms checkpoint data into custom data structures
- Runs with configurable concurrency (FANOUT)
- Returns `Vec<Value>` for each checkpoint

**Handler Trait**
- Batches values from multiple checkpoints
- Commits batches to database
- Handles conflicts with `ON CONFLICT` clauses

**Sequential Pipeline**
- Processes checkpoints in order
- Ensures data consistency
- Optimizes batch sizes automatically

## Handlers Implemented

### 1. TransactionDigestHandler
- **Purpose**: Extract and store transaction digests
- **Data**: Transaction hash + checkpoint sequence
- **Use**: Track all transactions processed

### 2. DataPodEventHandler (Template)
- **Purpose**: Extract DataPod smart contract events
- **Data**: Event type, datapod_id, seller, pricing info
- **Events**: Created, Published, Delisted, PriceUpdated

### 3. SmartContractObjectHandler (Template)
- **Purpose**: Track smart contract object state changes
- **Data**: Object ID, type, owner, version, digest
- **Use**: Monitor object lifecycle and ownership

## Getting Started

### Quick Start (5 minutes)

```bash
# 1. Create database
createdb sui_indexer

# 2. Configure environment
cp .env.example .env
# Edit .env with your database URL

# 3. Build and run
cargo build --release
cargo run --release
```

### Detailed Setup

See [SETUP.md](SETUP.md) for:
- PostgreSQL installation
- Environment configuration
- Troubleshooting
- Production deployment

### Quick Reference

See [QUICKSTART.md](QUICKSTART.md) for:
- Common commands
- Verification steps
- Quick troubleshooting

## Running the Indexer

### Basic Usage

```bash
cargo run --release
```

### With Options

```bash
# Start from checkpoint 1000
cargo run --release -- --first-checkpoint 1000

# Use Mainnet
cargo run --release -- --remote-store-url https://checkpoints.mainnet.sui.io

# Specify range
cargo run --release -- --first-checkpoint 1000 --last-checkpoint 2000

# View all options
cargo run --release -- --help
```

## Extending the Indexer

### Add a New Handler

1. Create a `Processor` implementation in `handlers.rs`
2. Implement the `Handler` trait for database commits
3. Register in `main.rs`:

```rust
cluster.sequential_pipeline(
    MyNewHandler,
    SequentialConfig::default(),
).await?;
```

### Add Database Tables

1. Create a new migration:
   ```bash
   diesel migration generate add_my_table
   ```

2. Edit migration files in `migrations/`

3. Update `models.rs` with new data structures

4. Update `schema.rs` with new table definitions

5. Run indexer - migrations apply automatically

## Database Queries

### Monitor Progress

```sql
-- Latest checkpoint processed
SELECT MAX(checkpoint_sequence_number) FROM transaction_digests;

-- Transaction count
SELECT COUNT(*) FROM transaction_digests;

-- Events by type
SELECT event_type, COUNT(*) FROM datapod_events GROUP BY event_type;
```

### Query DataPod Events

```sql
-- Find events by seller
SELECT * FROM datapod_events 
WHERE seller = '0x...' 
ORDER BY created_at DESC;

-- Price updates
SELECT * FROM datapod_events 
WHERE event_type = 'DataPodPriceUpdated' 
ORDER BY created_at DESC;
```

## Configuration

### Environment Variables (.env)

```
DATABASE_URL=postgres://username@localhost:5432/sui_indexer
```

### Optional Variables

```
CHECKPOINT_STORE_URL=https://checkpoints.testnet.sui.io
FIRST_CHECKPOINT=0
REMOTE_STORE_URL=https://checkpoints.testnet.sui.io
```

## Performance Characteristics

- **Sequential Processing**: Ensures data consistency
- **Automatic Batching**: Optimizes database writes
- **Configurable Concurrency**: FANOUT parameter controls worker threads
- **Smart Lag Handling**: Configurable checkpoint lag for optimal performance

## Monitoring

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run --release
```

### Check Database

```bash
psql sui_indexer -c "SELECT COUNT(*) FROM transaction_digests;"
```

### Performance Metrics

- Transactions per checkpoint
- Events per checkpoint
- Database write latency
- Checkpoint processing rate

## Documentation References

- [README.md](README.md) - Full architecture and extending guide
- [SETUP.md](SETUP.md) - Detailed installation and troubleshooting
- [QUICKSTART.md](QUICKSTART.md) - 5-minute quick start
- [Sui Documentation](https://docs.sui.io/guides/developer/advanced/custom-indexer) - Official docs
- [Diesel Documentation](https://diesel.rs/) - ORM reference

## Next Steps

1. **Build the Project**
   ```bash
   cargo build --release
   ```

2. **Set Up Database**
   ```bash
   createdb sui_indexer
   cp .env.example .env
   # Edit .env with your database URL
   ```

3. **Run the Indexer**
   ```bash
   cargo run --release
   ```

4. **Monitor Progress**
   ```bash
   psql sui_indexer -c "SELECT MAX(checkpoint_sequence_number) FROM transaction_digests;"
   ```

5. **Customize** (optional)
   - Add new handlers for different events
   - Extend database schema
   - Implement custom business logic

## Troubleshooting

### Build Issues
- Run `cargo clean && cargo build --release`
- Check Rust version: `rustc --version`

### Database Issues
- Verify PostgreSQL is running: `pg_isready`
- Check connection: `psql sui_indexer -c "SELECT 'Connected';"`

### Runtime Issues
- Enable debug logging: `RUST_LOG=debug cargo run --release`
- Check database permissions
- Verify .env file is correct

See [SETUP.md](SETUP.md) for detailed troubleshooting.

## Production Considerations

- Use managed PostgreSQL service (AWS RDS, Azure Database, etc.)
- Set up monitoring and alerting
- Configure automatic restarts (systemd, Docker)
- Enable database backups
- Monitor disk space and database size
- Use environment variables for sensitive data

See [SETUP.md](SETUP.md) for production deployment example.

## Support

- Check [Sui Documentation](https://docs.sui.io/)
- Review [Sui GitHub Issues](https://github.com/MystenLabs/sui/issues)
- Consult [Diesel Documentation](https://diesel.rs/)
- Enable debug logging for troubleshooting

---

**Status**: ✅ Ready to build and run

**Next Action**: Follow [QUICKSTART.md](QUICKSTART.md) to get started in 5 minutes
