# Sui Smart Contract Indexer - Setup Guide

This guide walks you through setting up and running the SourceNet Sui blockchain indexer for smart contracts.

## Prerequisites Checklist

- [ ] Rust 1.70+ installed (`rustc --version`)
- [ ] Cargo installed (`cargo --version`)
- [ ] PostgreSQL 12+ installed and running (`psql --version`)
- [ ] Git installed

## Step 1: Install Rust (if not already installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## Step 2: Install PostgreSQL

### On macOS (using Homebrew):
```bash
brew install postgresql@15
brew services start postgresql@15
```

### On Ubuntu/Debian:
```bash
sudo apt-get update
sudo apt-get install postgresql postgresql-contrib
sudo systemctl start postgresql
```

### On Windows:
Download and install from https://www.postgresql.org/download/windows/

## Step 3: Create PostgreSQL Database

Open a terminal and run:

```bash
# Create the database
createdb sui_indexer

# Verify the connection
psql sui_indexer -c "\conninfo"
```

Expected output:
```
You are connected to database "sui_indexer" as user "username" via socket in "/tmp" at port "5432".
```

### If you get a role error:

```bash
# Create the PostgreSQL user
sudo -u postgres createuser --superuser $(whoami)

# Then try creating the database again
createdb sui_indexer
```

## Step 4: Configure Environment Variables

In the project root directory:

```bash
# Copy the example environment file
cp .env.example .env

# Edit .env with your database credentials
# Replace 'username' with your PostgreSQL username
```

Your `.env` should look like:
```
DATABASE_URL=postgres://username@localhost:5432/sui_indexer
```

## Step 5: Install Diesel CLI (Optional but Recommended)

```bash
cargo install diesel_cli --no-default-features --features postgres
```

This allows you to manage migrations manually if needed.

## Step 6: Build the Indexer

```bash
# Navigate to the project directory
cd sourcenet-indexer

# Build the release binary
cargo build --release
```

This may take 5-15 minutes on first build due to dependency compilation.

## Step 7: Run the Indexer

### Option A: Using Cargo

```bash
cargo run --release
```

### Option B: Using the Compiled Binary

```bash
./target/release/sourcenet-indexer
```

### With Custom Options

```bash
# Start from checkpoint 1000
cargo run --release -- --first-checkpoint 1000

# Use Mainnet instead of Testnet
cargo run --release -- --remote-store-url https://checkpoints.mainnet.sui.io

# Specify checkpoint range
cargo run --release -- --first-checkpoint 1000 --last-checkpoint 2000

# View all available options
cargo run --release -- --help
```

## Step 8: Verify the Indexer is Working

In a new terminal, check the database:

```bash
# Connect to the database
psql sui_indexer

# Check transaction digests
SELECT COUNT(*) FROM transaction_digests;

# Check latest checkpoint processed
SELECT MAX(checkpoint_sequence_number) FROM transaction_digests;

# Exit psql
\q
```

## Troubleshooting

### Issue: "DATABASE_URL must be set in the environment"

**Solution**: 
1. Verify `.env` file exists in the project root
2. Check that `DATABASE_URL` is set correctly
3. Try running: `export $(cat .env | xargs) && cargo run --release`

### Issue: "connection to server on socket failed"

**Solution**:
1. Verify PostgreSQL is running: `pg_isready`
2. Check PostgreSQL service: `sudo systemctl status postgresql`
3. Start PostgreSQL if needed: `sudo systemctl start postgresql`

### Issue: "role 'username' does not exist"

**Solution**:
```bash
# Create the role
sudo -u postgres createuser --superuser $(whoami)

# Or specify a different username in DATABASE_URL
```

### Issue: "permission denied for schema public"

**Solution**:
```bash
psql sui_indexer -c "GRANT ALL ON SCHEMA public TO $(whoami);"
```

### Issue: Compilation errors with dependencies

**Solution**:
```bash
# Clean and rebuild
cargo clean
cargo build --release

# Or update dependencies
cargo update
```

### Issue: "too many open files"

**Solution**:
```bash
# Increase file descriptor limit
ulimit -n 4096

# Or permanently in ~/.bashrc or ~/.zshrc
echo "ulimit -n 4096" >> ~/.bashrc
```

## Next Steps

1. **Monitor Progress**: Check the database periodically to see indexed data
2. **Customize Handlers**: Modify `src/handlers.rs` to index additional events
3. **Extend Schema**: Add new tables in `migrations/` for custom data
4. **Deploy**: Set up systemd service or Docker container for production

## Database Queries

### View Transaction Count by Checkpoint

```sql
SELECT checkpoint_sequence_number, COUNT(*) as tx_count
FROM transaction_digests
GROUP BY checkpoint_sequence_number
ORDER BY checkpoint_sequence_number DESC
LIMIT 10;
```

### View DataPod Events

```sql
SELECT event_type, COUNT(*) as count
FROM datapod_events
GROUP BY event_type;
```

### Find Events by Seller

```sql
SELECT * FROM datapod_events
WHERE seller = '0x...' -- Replace with actual address
ORDER BY created_at DESC;
```

## Performance Tips

1. **Increase Batch Size**: Modify `SequentialConfig` in `main.rs` for faster processing
2. **Add Indexes**: Database indexes are created automatically by migrations
3. **Monitor Lag**: Check `checkpoint_lag` parameter in `SequentialConfig`
4. **Connection Pool**: Adjust PostgreSQL `max_connections` if needed

## Getting Help

- Check the [README.md](README.md) for architecture details
- Review [Sui Documentation](https://docs.sui.io/guides/developer/advanced/custom-indexer)
- Check PostgreSQL logs: `tail -f /var/log/postgresql/postgresql.log`
- Enable debug logging: `RUST_LOG=debug cargo run --release`

## Production Deployment

For production use:

1. Use a managed PostgreSQL service (AWS RDS, Azure Database, etc.)
2. Set up monitoring and alerting
3. Configure automatic restarts with systemd or Docker
4. Use environment variables for sensitive configuration
5. Enable PostgreSQL backups
6. Monitor disk space and database size

Example systemd service file:

```ini
[Unit]
Description=Sui Blockchain Indexer
After=network.target postgresql.service

[Service]
Type=simple
User=indexer
WorkingDirectory=/opt/sourcenet-indexer
EnvironmentFile=/opt/sourcenet-indexer/.env
ExecStart=/opt/sourcenet-indexer/target/release/sourcenet-indexer
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

## Additional Resources

- [Sui Official Documentation](https://docs.sui.io/)
- [Sui GitHub Repository](https://github.com/MystenLabs/sui)
- [Diesel ORM Guide](https://diesel.rs/)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
