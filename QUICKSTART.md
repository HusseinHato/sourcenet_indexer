# Quick Start Guide

Get the Sui smart contract indexer running in 5 minutes.

## TL;DR

```bash
# 1. Create database
createdb sui_indexer

# 2. Configure environment
cp .env.example .env
# Edit .env and set DATABASE_URL

# 3. Build and run
cargo run --release
```

## Detailed Steps

### 1. Prerequisites

Ensure you have:
- Rust installed: `rustc --version`
- PostgreSQL running: `psql --version`

If not, see [SETUP.md](SETUP.md) for installation instructions.

### 2. Database Setup

```bash
# Create the database
createdb sui_indexer

# Verify connection
psql sui_indexer -c "SELECT 'Connected';"
```

### 3. Environment Configuration

```bash
# Copy example configuration
cp .env.example .env

# Edit .env with your database URL
# For local PostgreSQL:
# DATABASE_URL=postgres://username@localhost:5432/sui_indexer
```

### 4. Build the Project

```bash
cargo build --release
```

First build takes 5-15 minutes. Subsequent builds are faster.

### 5. Run the Indexer

```bash
cargo run --release
```

The indexer will:
1. Apply database migrations automatically
2. Connect to Sui Testnet checkpoint store
3. Start processing checkpoints sequentially
4. Store transaction data in PostgreSQL

### 6. Verify It's Working

In another terminal:

```bash
# Check how many transactions have been indexed
psql sui_indexer -c "SELECT COUNT(*) FROM transaction_digests;"

# Check the latest checkpoint processed
psql sui_indexer -c "SELECT MAX(checkpoint_sequence_number) FROM transaction_digests;"
```

## Common Commands

```bash
# Run with debug logging
RUST_LOG=debug cargo run --release

# Start from a specific checkpoint
cargo run --release -- --first-checkpoint 1000

# Use Mainnet instead of Testnet
cargo run --release -- --remote-store-url https://checkpoints.mainnet.sui.io

# View all available options
cargo run --release -- --help

# Stop the indexer
# Press Ctrl+C in the terminal
```

## Troubleshooting

**"DATABASE_URL must be set"**
- Make sure `.env` file exists in the project root
- Verify `DATABASE_URL` is set correctly

**"connection to server failed"**
- Check PostgreSQL is running: `pg_isready`
- Start PostgreSQL if needed

**"role does not exist"**
- Create the user: `sudo -u postgres createuser --superuser $(whoami)`

For more help, see [SETUP.md](SETUP.md) or [README.md](README.md).

## Next Steps

1. **Monitor Progress**: Query the database to see indexed data
2. **Customize**: Modify `src/handlers.rs` to index different events
3. **Deploy**: Set up for production use

## Architecture Overview

```
Sui Checkpoint Store
        ↓
    Indexer
        ↓
  Processor (extract data)
        ↓
  Handler (batch & commit)
        ↓
  PostgreSQL Database
```

The indexer processes checkpoints sequentially, extracting transaction data and smart contract events, then stores them in PostgreSQL for efficient querying.

## File Structure

```
src/
├── main.rs      - Entry point, sets up indexer cluster
├── models.rs    - Data structures for database records
├── handlers.rs  - Processor and Handler implementations
└── schema.rs    - Diesel ORM schema

migrations/
└── 2024-01-01-000000_create_smart_contract_events/
    ├── up.sql   - Create tables
    └── down.sql - Drop tables
```

## Database Tables

- **transaction_digests**: Transaction hashes from checkpoints
- **datapod_events**: Events from DataPod smart contract
- **smart_contract_objects**: Smart contract object states

## Performance

- Sequential processing ensures data consistency
- Automatic batching optimizes database writes
- Indexes on common queries for fast lookups

## Documentation

- [Full README](README.md) - Architecture and extending
- [Setup Guide](SETUP.md) - Detailed installation
- [Sui Docs](https://docs.sui.io/guides/developer/advanced/custom-indexer) - Official documentation

---

**Ready to go!** Start indexing Sui smart contracts now.
