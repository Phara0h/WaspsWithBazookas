# 🎯 Test Dummy Server (httpbin-style)

> *"The fastest, simplest test target for your wasp army!"*

---

## 🚀 What is the Test Dummy Server?

The **Test Dummy Server** is a lightning-fast, zero-dependency HTTP server built with `may_minihttp` that mimics the best of [httpbin](https://httpbin.org/). It's designed to be the perfect target for load testing with WaspsWithBazookas, with ultra-fast responses and classic endpoints for every scenario.

---

## 🌟 Why Use This Test Dummy?

- **Blazing Fast**: `/` route is as fast as possible—no allocations, just a static string.
- **httpbin-style Endpoints**: Test status codes, delays, headers, echo, UUIDs, and more.
- **No External Dependencies**: Just run the binary, no config, no setup.
- **Predictable, Consistent**: Every response is simple and reliable.
- **Perfect for Load Testing**: Designed to be hammered by your wasp army!
- **Configurable**: Set custom port, host, and protocol (HTTP/HTTPS).

---

## 🚀 Quick Start

```bash
# Build and run the test dummy server
cargo build --release --bin test-dummy
./target/release/test-dummy

# Or if installed via cargo install
test-dummy

# With custom configuration
test-dummy --port 9000 --host 0.0.0.0
```

---

## ⚙️ Configuration Options

### **Command Line Arguments**

```bash
test-dummy [OPTIONS]

Options:
  -p, --port <PORT>              Port to listen on (default: 8080)
  -i, --host <HOST>              Host/IP to bind to (default: 127.0.0.1)
  --https                        Enable HTTPS (requires certificate files)
  --cert <CERT>                  Path to SSL certificate file (PEM format)
  --key <KEY>                    Path to SSL private key file (PEM format)
  -h, --help                     Print help
  -V, --version                  Print version
```

### **Usage Examples**

```bash
# Default configuration (HTTP on 127.0.0.1:8080)
test-dummy

# Custom port
test-dummy --port 9000

# Bind to all interfaces
test-dummy --host 0.0.0.0 --port 8080

# HTTPS with certificates (requires different HTTP server)
test-dummy --https --cert cert.pem --key key.pem

# Short form options
test-dummy -i 0.0.0.0 -p 9000
```

---

## 📋 Available Endpoints

| Method | Endpoint            | Description                                 | Status Code |
|--------|---------------------|---------------------------------------------|-------------|
| GET    | `/`                 | Fastest possible static response            | 200         |
| GET    | `/get`              | Echo method and path as JSON                | 200         |
| GET    | `/status/:code`     | Returns the given status code               | :code       |
| GET    | `/delay/:seconds`   | Waits N seconds, then returns               | 200         |
| GET    | `/headers`          | Returns request headers as JSON             | 200         |
| GET    | `/ip`               | Returns a fake client IP as JSON            | 200         |
| GET    | `/uuid`             | Returns a random UUID as JSON               | 200         |
| ANY    | `/anything`         | Echoes method, path, headers, and body      | 200         |

---

## 📝 Endpoint Details

### `/` (Ultra-fast root)
- **GET** `/`
- Returns: `OK` (plain text)
- **Purpose:** Fastest possible response for RPS/latency benchmarks

### `/get`
- **GET** `/get`
- Returns: `{ "method": "GET", "path": "/get" }`
- **Purpose:** Echoes request info as JSON

### `/status/:code`
- **GET** `/status/404` (or any status code)
- Returns: Empty body, status code set to `:code`
- **Purpose:** Test error handling, status code parsing, etc.

### `/delay/:seconds`
- **GET** `/delay/2`
- Waits up to 10 seconds, then returns `delayed`
- **Purpose:** Simulate slow endpoints

### `/headers`
- **GET** `/headers`
- Returns: `{ "headers": { ... } }` (all request headers)
- **Purpose:** Test header propagation

### `/ip`
- **GET** `/ip`
- Returns: `{ "origin": "127.0.0.1" }`
- **Purpose:** Test client IP logic

### `/uuid`
- **GET** `/uuid`
- Returns: `{ "uuid": "..." }` (random UUID)
- **Purpose:** Test randomness, unique values

### `/anything`
- **ANY** `/anything`
- Returns: `{ "method": ..., "path": ..., "headers": ..., "body": ... }`
- **Purpose:** Echoes everything about the request (method, path, headers, body)

---

## 🚀 Example Usage

```bash
# Start server on default port
test-dummy

# Test the endpoints
curl http://localhost:8080/

# Start on different port and test
test-dummy --port 9000
curl http://localhost:9000/get

# Start on all interfaces for remote access
test-dummy --host 0.0.0.0 --port 8080
curl http://your-server-ip:8080/headers

# Test with custom headers
curl -H "X-Test: 123" http://localhost:8080/headers

# Test status codes
curl -i http://localhost:8080/status/404

# Test delays
curl http://localhost:8080/delay/2

# Test POST with body
curl -X POST -d 'hello' http://localhost:8080/anything
```

---

## 🔒 HTTPS Support

**Note:** The current implementation using `may_minihttp` doesn't support HTTPS directly. The `--https`, `--cert`, and `--key` options are provided for future compatibility but will show an error message.

For HTTPS testing, consider:
- Using a reverse proxy (nginx, haproxy)
- Using a different HTTP server implementation
- Testing HTTP endpoints and handling HTTPS at the load balancer level

---

## 🎯 Load Testing Scenarios

- **RPS Benchmark:** Use `/` for the highest possible throughput test
- **Latency Test:** Use `/delay/:seconds` to simulate slow endpoints
- **Error Handling:** Use `/status/500` or `/status/404` to test error paths
- **Header Propagation:** Use `/headers` to verify custom headers
- **Echo/Debug:** Use `/anything` to see exactly what your wasps are sending

---

## 🔧 Customization

Want to add more endpoints? Just edit `src/test_dummy.rs` and follow the simple match logic. No frameworks, no magic—just Rust and speed.

---

## 🎉 Happy Load Testing!

The Test Dummy Server is now the perfect, ultra-fast, zero-dependency target for your WaspsWithBazookas swarm. Hammer it, break it, and enjoy the speed!

---

## 📚 Related Documentation

- **[Installation Guide](docs/INSTALL.md)**
- **[Usage Guide](docs/RUN.md)**
- **[API Documentation](docs/API.md)** 