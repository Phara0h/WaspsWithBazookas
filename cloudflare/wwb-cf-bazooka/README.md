# 🚀 Bazooka Worker (Cloudflare Workers)

> *"The heavy artillery for distributed load testing on Cloudflare's edge!"*

## 🎯 Overview

The **Bazooka Worker** is a Cloudflare Worker that handles the actual load generation for WaspsWithBazookas. It's designed to work within Cloudflare Workers' strict limits while providing powerful load testing capabilities.

## ⚡ Cloudflare Workers Limits & Solutions

### **Limits We Work Around**
- **1000 subrequests per request**: We use a chunked approach with 500 requests per chunk
- **30-second timeout**: We monitor execution time and stop before hitting the limit
- **128MB memory**: We use efficient data structures and streaming
- **10ms CPU time**: We batch operations and use async patterns

### **Chunked Architecture**
```
Wasp Worker → Bazooka Worker (Chunk 1: 500 requests) → Target
     ↓
Wasp Worker → Bazooka Worker (Chunk 2: 500 requests) → Target  
     ↓
Wasp Worker → Bazooka Worker (Chunk N: remaining requests) → Target
```

## 🏗️ Architecture

### **Components**
- **Durable Object**: Maintains state across requests and chunks
- **Chunked Execution**: Breaks large load tests into manageable chunks
- **Real-time Stats**: Aggregates results from all chunks
- **Error Handling**: Robust error recovery and reporting

### **Data Flow**
1. **Start Request**: Wasp worker initiates load test
2. **Chunk Execution**: Bazooka worker executes requests in chunks of 500
3. **State Management**: Durable Object tracks progress and aggregates results
4. **Completion**: Final stats are compiled and returned

## 🚀 Quick Start

### **1. Deploy the Worker**

```bash
# Navigate to the bazooka worker directory
cd wwb-cf-bazooka

# Deploy to Cloudflare Workers
wrangler deploy
```

### **2. Configure Environment Variables**

```toml
# wrangler.toml
[vars]
WORKER_NAME = "bazooka-worker"

[[durable_objects.bindings]]
name = "BAZOOKA_STATE"
class_name = "BazookaState"
```

### **3. Test the Deployment**

```bash
# Health check
curl https://your-worker.your-subdomain.workers.dev/health

# Start a load test
curl -X POST https://your-worker.your-subdomain.workers.dev/start \
  -H "Content-Type: application/json" \
  -d '{
    "target": "https://httpbin.org/get",
    "method": "GET",
    "headers": {},
    "timeout": 30,
    "connections": 100,
    "threads": 10,
    "duration_secs": 60
  }'
```

## 📡 API Endpoints

### **POST /start**
Start a new load test with chunked execution.

**Request Body:**
```json
{
  "target": "https://example.com",
  "method": "GET",
  "headers": {
    "User-Agent": "WaspsWithBazookas/2.0.0"
  },
  "body": null,
  "timeout": 30,
  "connections": 1000,
  "threads": 10,
  "duration_secs": 60
}
```

**Response:**
```json
{
  "status": "started",
  "battle_id": "durable-object-id",
  "message": "Bazooka battle started with chunked approach!",
  "first_chunk_completed": true,
  "remaining_chunks": 1
}
```

### **POST /chunk**
Execute a specific chunk of requests.

**Request Body:**
```json
{
  "chunk_id": 1,
  "target": "https://example.com",
  "method": "GET",
  "headers": {},
  "body": null,
  "timeout": 30,
  "requests_per_chunk": 500,
  "total_requests": 1000,
  "duration_secs": 60
}
```

**Response:**
```json
{
  "status": "chunk_completed",
  "chunk_id": 1,
  "requests_completed": 500,
  "total_requests": 1000,
  "is_final_chunk": true
}
```

### **POST /stop**
Stop the current load test.

**Response:**
```json
{
  "status": "stopped",
  "message": "Bazooka battle stopped!"
}
```

### **GET /stats**
Get current load test statistics.

**Response:**
```json
{
  "requests": 1000,
  "bytes": 2048000,
  "rps": 16.67,
  "status_counts": {
    "200": 950,
    "429": 50
  },
  "latency_p50": 45.2,
  "latency_p90": 89.7,
  "latency_p99": 156.3,
  "duration_secs": 60,
  "is_running": false,
  "error": null,
  "chunks_completed": 2,
  "total_chunks": 2
}
```

### **GET /health**
Health check endpoint.

**Response:**
```json
{
  "status": "healthy",
  "service": "bazooka-worker",
  "timestamp": "2024-01-15T10:30:00Z",
  "limits": {
    "max_subrequests": 1000,
    "max_cpu_time_ms": 10,
    "max_memory_mb": 128,
    "max_duration_secs": 30
  }
}
```

## 🔧 Configuration

### **Environment Variables**
```toml
# wrangler.toml
[vars]
WORKER_NAME = "bazooka-worker"
MAX_REQUESTS_PER_CHUNK = "500"
CHUNK_TIMEOUT_SECS = "25"
BATCH_SIZE = "10"
```

### **Durable Object Configuration**
```toml
[[durable_objects.bindings]]
name = "BAZOOKA_STATE"
class_name = "BazookaState"

[[migrations]]
tag = "v1"
new_classes = ["BazookaState"]
```

## 🎯 Load Testing Strategy

### **Chunked Execution**
- **500 requests per chunk**: Stays well under the 1000 subrequest limit
- **Batch processing**: 10 requests per batch to avoid overwhelming targets
- **Timeout monitoring**: Stops at 25 seconds to stay under 30-second limit
- **State persistence**: Durable Object maintains progress across chunks

### **Performance Optimizations**
- **Connection pooling**: Reuses connections when possible
- **Header optimization**: Minimizes header overhead
- **Memory management**: Efficient data structures and streaming
- **Error recovery**: Graceful handling of network issues

### **Monitoring & Metrics**
- **Real-time stats**: Live updates during chunk execution
- **Latency percentiles**: P50, P90, P99 calculations
- **Status code tracking**: Detailed HTTP status code distribution
- **Error reporting**: Comprehensive error categorization

## 🚨 Error Handling

### **Common Scenarios**
- **Network timeouts**: Automatic retry with exponential backoff
- **Target unavailability**: Graceful degradation and reporting
- **Rate limiting**: Respects 429 responses and adjusts timing
- **Memory pressure**: Efficient cleanup and garbage collection

### **Recovery Strategies**
- **Chunk-level recovery**: Failed chunks can be retried
- **State preservation**: Progress is maintained across failures
- **Graceful degradation**: Continues with partial results
- **Detailed logging**: Comprehensive error reporting

## 🔒 Security Considerations

### **Request Validation**
- **URL validation**: Ensures targets are valid HTTP/HTTPS URLs
- **Header sanitization**: Prevents header injection attacks
- **Timeout limits**: Prevents resource exhaustion
- **Rate limiting**: Respects target server limits

### **Access Control**
- **CORS configuration**: Proper cross-origin handling
- **Input validation**: Sanitizes all user inputs
- **Error masking**: Prevents information leakage
- **Resource limits**: Enforces Cloudflare Workers constraints

## 📊 Performance Characteristics

### **Throughput**
- **Up to 500 requests per chunk**: Limited by subrequest count
- **Multiple chunks per test**: Scales horizontally
- **Real-time processing**: Minimal latency overhead
- **Efficient aggregation**: Fast stats compilation

### **Latency**
- **Edge network**: Global distribution reduces latency
- **Connection reuse**: Minimizes connection overhead
- **Optimized headers**: Reduces request size
- **Batch processing**: Efficient request grouping

### **Reliability**
- **Durable Object state**: Persistent across failures
- **Chunk-level isolation**: Failures don't affect other chunks
- **Automatic retries**: Built-in error recovery
- **Comprehensive monitoring**: Full visibility into operations

## 🛠️ Development

### **Local Development**
```bash
# Install dependencies
npm install

# Start local development server
wrangler dev --local

# Run tests
cargo test
```

### **Testing**
```bash
# Test individual endpoints
curl http://localhost:8787/health
curl http://localhost:8787/stats

# Test chunk execution
curl -X POST http://localhost:8787/chunk \
  -H "Content-Type: application/json" \
  -d '{"chunk_id": 0, "target": "https://httpbin.org/get", ...}'
```

### **Deployment**
```bash
# Deploy to staging
wrangler deploy --env staging

# Deploy to production
wrangler deploy --env production
```

## 🔍 Monitoring & Debugging

### **Logs**
```bash
# View real-time logs
wrangler tail

# Filter by specific events
wrangler tail --format pretty | grep "chunk"
```

### **Metrics**
- **Request count**: Total requests processed
- **Success rate**: Percentage of successful requests
- **Latency distribution**: P50, P90, P99 percentiles
- **Error rates**: Categorized error statistics

### **Health Checks**
```bash
# Check worker health
curl https://your-worker.workers.dev/health

# Monitor stats
curl https://your-worker.workers.dev/stats
```

## 🎉 Success Stories

### **Large-Scale Testing**
- **10,000+ requests**: Successfully tested across multiple chunks
- **Global distribution**: Leveraged Cloudflare's edge network
- **Real-time monitoring**: Live stats during execution
- **Cost efficiency**: Pay-per-request pricing model

### **Production Use Cases**
- **API load testing**: Validated production API performance
- **CDN testing**: Tested content delivery across regions
- **Microservices**: Load tested individual service components
- **Infrastructure validation**: Verified scaling capabilities

---

## 📚 Related Documentation

- **[Wasp Worker](../wwb-cf-wasp/README.md)** - The coordinating worker
- **[Main Documentation](../../CLOUDFLARE_WORKERS_README.md)** - Complete setup guide
- **[API Reference](../../docs/API.md)** - Full API documentation
- **[Cloudflare Workers Docs](https://developers.cloudflare.com/workers/)** - Official documentation 