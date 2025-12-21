# Railway Setup

## Required Environment Variables

### Required

1. **RPC_HTTP_URL**
   - URL of the blockchain RPC node
   - Example: `http://216.106.182.100:32774`

2. **ES_URL**
   - URL of your Elasticsearch instance on Railway
   - If using Railway Elasticsearch, get the URL from the service's environment variables
   - Example: `https://your-elasticsearch.railway.app:9200` or `http://localhost:9200` if in the same project

### Optional (with default values)

3. **ES_USERNAME** (optional)
   - Elasticsearch username if authentication is required
   - Example: `elastic`

4. **ES_PASSWORD** (optional)
   - Elasticsearch password
   - Example: `your_secret_password`

5. **INDEX_PREFIX** (default: `workqueue`)
   - Prefix for Elasticsearch indices
   - Will create indices: `workqueue-blocks` and `workqueue-meta`

6. **BATCH_SIZE** (default: `1000`)
   - Number of blocks to process per batch
   - Higher values = faster but more memory usage

7. **START_BLOCK** (default: `0`)
   - Block number to start indexing from
   - `0` = from genesis block
   - Useful to resume from a specific block

8. **SYNC_INTERVAL_SECS** (default: `2`)
   - Seconds between checks in live sync mode
   - Lower values = more frequent but more RPC load

9. **CONCURRENCY** (default: `10`)
   - Number of concurrent tasks to process blocks
   - Higher values = faster but more RPC load

10. **ES_BULK_SIZE** (default: `100`)
    - Number of blocks to index in a single bulk operation
    - Higher values = faster but more memory usage

## Deployment Steps

1. **Create a new Railway project**
   - Go to [railway.app](https://railway.app)
   - Create a new project
   - Connect your GitHub repository

2. **Add Elasticsearch service (if you don't have one)**
   - In Railway, add a new service
   - Select "Elasticsearch" from the marketplace
   - Railway will automatically configure environment variables

3. **Configure Environment Variables**
   - In the Railway dashboard, go to your indexer service
   - Go to the "Variables" tab
   - Add all environment variables listed above
   - **IMPORTANT**: Do not include quotes in the values

4. **Railway Reference Variables (if Elasticsearch is in the same project)**
   - If Elasticsearch is in the same Railway project, you can use:
   - `ES_URL` = Public URL that Railway assigns to your Elasticsearch service
   - Or the internal URL if Railway exposes it: `http://elasticsearch:9200`
   - Check the Elasticsearch service's environment variables

5. **Deploy**
   - Railway will automatically detect it's a Rust project
   - Build will run automatically with `cargo build --release`
   - Service will start with `./target/release/blockchain-indexer`
   - The indexer will run continuously

## Indexer Behavior

The indexer works in two phases:

1. **Historical Sync**: Indexes all blocks from `START_BLOCK` to the current block
   - Processes blocks in batches of `BATCH_SIZE`
   - Saves checkpoints after each batch
   - Shows progress, speed, and ETA in logs

2. **Live Sync**: Continuously monitors new blocks and indexes them automatically
   - Checks every `SYNC_INTERVAL_SECS` seconds for new blocks
   - Indexes new blocks automatically
   - Runs indefinitely

The indexer saves checkpoints in Elasticsearch, so if it restarts, it will continue from the last indexed block.

## Verification

Once deployed, you can verify it's working:

1. **Check logs in Railway**
   - You should see messages like:
     - "Initializing Blockchain Indexer..."
     - "Starting historical sync..."
     - "Processing batch: blocks X to Y"
     - "Live sync completed: now at block X"

2. **Verify in Elasticsearch that indices are being created:**
   - `workqueue-blocks` (or `{INDEX_PREFIX}-blocks`)
   - `workqueue-meta` (or `{INDEX_PREFIX}-meta`)

3. **Query the indexed data:**
   ```bash
   # Verify that blocks are indexed
   curl -X GET "ES_URL/workqueue-blocks/_count"
   
   # See the last checkpoint
   curl -X GET "ES_URL/workqueue-meta/_doc/checkpoint"
   ```

## Troubleshooting

**Elasticsearch connection error:**
- Verify that `ES_URL` is correct
- If Elasticsearch is on Railway, make sure to use the correct URL (public or internal)
- Verify credentials if configured
- Make sure Elasticsearch is accessible from the indexer service

**RPC connection error:**
- Verify that `RPC_HTTP_URL` is accessible from Railway
- Some RPCs may have IP restrictions
- Verify that the RPC is working correctly

**Indexer keeps restarting:**
- Check logs to see the specific error
- Verify that all required environment variables are configured correctly
- Railway has `restartPolicyType: ON_FAILURE` configured, so it will automatically restart if it fails

**Indexer is not progressing:**
- Verify that the RPC is responding correctly
- Check logs to see if there are rate limiting errors
- Consider increasing `SYNC_INTERVAL_SECS` if the RPC has limits

**Insufficient memory:**
- Reduce `BATCH_SIZE` or `ES_BULK_SIZE`
- Reduce `CONCURRENCY`
- Railway may need a plan with more resources

## Important Notes

- The indexer runs **continuously** on Railway
- It will automatically restart if it fails (up to 10 retries)
- Checkpoints allow resuming from where it left off
- Live sync mode keeps the database updated in real-time
- Railway may pause inactive services on free plans, but the indexer is constantly active
