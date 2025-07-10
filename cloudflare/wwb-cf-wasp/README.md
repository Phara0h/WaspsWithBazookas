# 🐝 Wasp Worker - Cloudflare Wasp Agent

This is the **Wasp Worker** - a Cloudflare Worker that acts as a wasp agent for WaspsWithBazookas, coordinating with the Hive server and Bazooka worker.

## 🏗️ Architecture

The Wasp Worker consists of:

1. **API Endpoints**: Handles wasp commands (`/fire`, `/ceasefire`, `/die`, `/battlereport`, etc.)
2. **Hive Client**: Communicates with the Hive server for check-in, heartbeat, and reporting
3. **Bazooka Client**: Orchestrates load testing via the Bazooka worker
4. **State Management**: Maintains wasp identity and configuration
5. **Persistent Heartbeat**: Uses Durable Object alarms to maintain continuous heartbeat with Hive

## 🔄 Persistent Heartbeat System

The Wasp Worker uses Cloudflare Durable Objects with the Alarms API to maintain a persistent heartbeat with the Hive server:

### **How It Works**
- **Durable Object**: `WaspState` maintains wasp configuration and heartbeat state
- **Alarm Handler**: `alarm()` method is called every 5 seconds to send heartbeats
- **Automatic Initialization**: Heartbeat starts automatically when the worker first receives a request
- **Graceful Shutdown**: Heartbeat stops when `/die` is called

### **Heartbeat Flow**
```
Worker Startup → Initialize WaspState → Set 5-second alarm → Send heartbeat → Set next alarm → Repeat
```

### **Benefits**
- **Persistent**: Heartbeat continues even when no requests are being processed
- **Cost-effective**: Uses Durable Object alarms instead of continuous polling
- **Reliable**: Automatic retry with exponential backoff on failures
- **Clean shutdown**: Proper cleanup when wasp is terminated

## 🚀 Quick Start

### 1. Deploy the Worker

```bash
# Navigate to the wasp worker directory
cd wwb-cf-wasp

# Install dependencies
npm install -g wrangler

# Login to Cloudflare
wrangler login

# Deploy the worker
wrangler deploy
```

### 2. Configure Environment Variables

Set these environment variables in your Cloudflare dashboard or via wrangler:

```bash
# Required: Hive server URL
wrangler secret put HIVE_URL

# Required: Bazooka worker URL
wrangler secret put BAZOOKA_WORKER_URL

# Optional: Authentication tokens
wrangler secret put HIVE_TOKEN
wrangler secret put BAZOOKA_WORKER_TOKEN

# Optional: Wasp configuration
wrangler secret put WASP_ID
wrangler secret put WASP_PORT
```

### 3. Test the Worker

```bash
# Health check
curl https://your-wasp-worker.your-subdomain.workers.dev/health

# Boop the wasp
curl https://your-wasp-worker.your-subdomain.workers.dev/boop

# Fire a load test
curl -X PUT https://your-wasp-worker.your-subdomain.workers.dev/fire \
  -H "Content-Type: application/json" \
  -d '{
    "target": "https://httpbin.org/get",
    "t": 10,
    "c": 50,
    "d": 30,
    "method": "GET",
    "headers": {
      "User-Agent": "WaspsWithBazookas/2.0.0"
    }
  }'

# Get battle report
curl https://your-wasp-worker.your-subdomain.workers.dev/battlereport

# Ceasefire
curl https://your-wasp-worker.your-subdomain.workers.dev/ceasefire

# Die (self-destruct)
curl -X DELETE https://your-wasp-worker.your-subdomain.workers.dev/die
```

## 📋 API Endpoints

### GET `/boop`
Health check endpoint - returns a simple greeting.

**Response:**
```
Oh hi from Cloudflare Wasp!
```

### PUT `/fire`
Start a load test.

**Request Body:**
```json
{
  "target": "https://example.com",
  "t": 10,
  "c": 50,
  "d": 30,
  "timeout": 2,
  "method": "GET",
  "headers": {
    "User-Agent": "Custom-Agent"
  },
  "body": "optional request body"
}
```

**Response:**
```json
{
  "status": "200",
  "message": "🚀 I'M A'FIRIN' MAH ROCKETS! (Cloudflare style)"
}
```

### GET `/ceasefire`
Stop the currently running load test.

**Response:**
```json
{
  "status": "200",
  "message": "Ok i stops (Cloudflare style)"
}
```

### DELETE `/die`
Stop any running tests, stop the heartbeat loop, and prepare for shutdown.

**Response:**
```json
{
  "status": "shutting_down",
  "message": "Wasp is shutting down... Heartbeat stopped and state cleaned up.",
  "timestamp": "2024-01-01T12:00:00Z"
}
```

### GET `/battlereport`
Get the latest battle results.

**Response:**
```json
{
  "id": "CloudflareWasp",
  "timestamp": "2024-01-01T12:00:00Z",
  "url": "https://example.com",
  "method": "GET",
  "connections": 50,
  "duration": 30,
  "threads": 10,
  "requests": 1500,
  "bytes": 2048000,
  "rps": 50.0,
  "status_counts": {
    "200": "1450",
    "404": "50"
  },
  "latency_p50": 25.5,
  "latency_p90": 45.2,
  "latency_p99": 120.1,
  "raw_output": "..."
}
```

### GET `/health`
Comprehensive health check including Hive and Bazooka connectivity.

**Response:**
```json
{
  "hive": "ok",
  "bazooka": "ok",
  "wasp_id": "CloudflareWasp",
  "port": "443"
}
```

## 🔧 Durable Object Commands

The Wasp Worker uses Durable Object commands for internal communication:

### **Available Commands**
- `initialize_wasp`: Initialize wasp state and start heartbeat loop
- `stop_heartbeat`: Stop the heartbeat loop and clean up alarms
- `initialize_battle`: Set up battle state for load testing
- `bazooka_completed`: Handle completion of a Bazooka worker
- `stop_battle`: Stop the current battle
- `get_battle_stats`: Retrieve current battle statistics
- `cleanup_battle`: Clean up battle state to save costs

### **Command Format**
```json
{
  "command": "initialize_wasp",
  "data": {
    "wasp_id": "CloudflareWasp",
    "port": "443",
    "hive_url": "https://hive.example.com",
    "initialized_at": "2024-01-01T12:00:00Z"
  }
}
```

## 🔧 Configuration

### Environment Variables

- `HIVE_URL`: URL of the Hive server (required)
- `BAZOOKA_WORKER_URL`: URL of the Bazooka worker (required)
- `HIVE_TOKEN`: Authentication token for Hive (optional)
- `BAZOOKA_WORKER_TOKEN`: Authentication token for Bazooka worker (optional)
- `WASP_ID`: Unique identifier for this wasp (default: "CloudflareWasp")
- `WASP_PORT`: Port number for this wasp (default: "443")

### Wrangler Configuration

The `wrangler.toml` file configures:

- **Worker name**: `wwb-cf-wasp`
- **Compatibility date**: Latest Cloudflare Workers features
- **Build command**: Uses worker-build for Rust compilation

## 🔗 Integration Flow

### 1. Check-in with Hive
When the worker starts, it automatically checks in with the Hive server:

```
Wasp Worker → Hive Server (/wasp/checkin/:port)
```

### 2. Heartbeat
The worker sends periodic heartbeats to the Hive:

```
Wasp Worker → Hive Server (/wasp/heartbeat/:port)
```

### 3. Load Test Execution
When a `/fire` command is received:

1. Wasp Worker validates the request
2. Wasp Worker calls Bazooka Worker `/start` endpoint
3. Bazooka Worker executes the load test
4. Wasp Worker can query `/stats` for results
5. Wasp Worker reports results back to Hive

### 4. Reporting to Hive
After load test completion:

```
Wasp Worker → Hive Server (/wasp/reportin/:id)
```

## 🔒 Security

- CORS headers are automatically added to all responses
- Optional token-based authentication for both Hive and Bazooka
- Input validation on all endpoints
- Secure communication with external services

## 🚨 Limitations

- **No Persistent State**: Workers are stateless - state is maintained in Durable Objects (Bazooka worker)
- **Request Timeout**: Workers timeout after 30 seconds
- **Subrequest Limits**: Limited to 1000 subrequests per request
- **No Background Tasks**: Cannot run continuous background processes

## 🔧 Development

### Local Development

```bash
# Start local development server
wrangler dev

# Run tests (if implemented)
cargo test
```

### Building

```bash
# Build the worker
cargo build --release

# Deploy to production
wrangler deploy --release
```

## 🎯 Use Cases

### 1. Distributed Load Testing
Deploy multiple Wasp Workers across different Cloudflare regions for global load testing.

### 2. Serverless Load Testing
Use Cloudflare Workers for on-demand, scalable load testing without managing infrastructure.

### 3. Integration Testing
Integrate load testing into CI/CD pipelines using the REST API.

### 4. Monitoring and Alerting
Use the health endpoints for monitoring wasp availability.

## 🔗 Integration with Hive

The Wasp Worker integrates seamlessly with the existing Hive server:

- **Check-in**: Registers with Hive on startup
- **Heartbeat**: Maintains connection with periodic heartbeats
- **Reporting**: Sends battle results back to Hive
- **Authentication**: Supports token-based authentication

## 🎯 Next Steps

- Implement automatic retry logic for failed requests
- Add support for custom load testing scripts
- Implement more sophisticated error handling
- Add metrics and monitoring capabilities
- Support for multiple concurrent load tests
- Integration with Cloudflare Analytics 