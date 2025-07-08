# 🚀 Usage Guide

> *"Commanding your wasp army to wreak havoc on your servers!"*

---

## 🎯 Quick Start

Ready to unleash the wasps? Here's how to get started in under 5 minutes:

### **1. Start the Hive (Command Center)**

```bash
# Start the hive on default port 4269
hive

# Or specify a custom port
hive --port 8080

# With authentication (recommended for production)
hive --wwb-token "your-secret-token-here"
```

### **2. Deploy Your Wasp Army**

```bash
# Start a single wasp
wasp --hive-url http://hiveaddress:4269 --port 3001

# Start multiple wasps (in separate terminals) or installed on separate servers / machines
wasp --hive-url http://hiveaddress:4269 --port 3001
wasp --hive-url http://hiveaddress:4269 --port 3002
wasp --hive-url http://hiveaddress:4269 --port 3003

# With authentication
wasp --hive-url http://hiveaddress:4269 --port 3001 --wwb-token "your-secret-token-here"
```

### **3. Launch Your First Attack**

```bash
# Basic load test
curl -X PUT http://hiveaddress:4269/hive/poke \
  -H "Content-Type: application/json" \
  -d '{
    "target": "https://example.com",
    "t": 10,
    "c": 100,
    "d": 30
  }'

# With authentication
curl -X PUT http://hiveaddress:4269/hive/poke \
  -H "Content-Type: application/json" \
  -H "wwb-token: your-secret-token-here" \
  -d '{
    "target": "https://example.com",
    "t": 10,
    "c": 100,
    "d": 30
  }'
```

**🎉 Boom!** You just launched a distributed load test with your wasp army!

---

## 🏗️ Architecture Overview

```
    🏠 HIVE (Port 4269)
    Command Center & Coordinator
         ↕️ HTTP API
    🐝🐝🐝🐝🐝🐝🐝🐝
    Individual Wasp Agents
    (Ports 3001, 3002, etc.)
         ↕️ HTTP Requests
    🎯 TARGET SERVER
    (Your application under test)
```

### **Components**

- **🏠 Hive**: Your command center that coordinates all wasps
- **🐝 Wasps**: Individual load testing agents that do the heavy lifting
- **🎯 Target**: The server you want to test

---

## 🎮 Command Line Interface

### **Hive Commands**

```bash
# Basic usage
hive [OPTIONS]

# Options
--port <PORT>              # Port to listen on (default: 4269)
--host <HOST>              # Host to bind to (default: 0.0.0.0)
--wwb-token <TOKEN>        # Authentication token
--log <LOG_FILE>           # Log file path
```

### **Wasp Commands**

```bash
# Basic usage
wasp [OPTIONS]

# Options
--port <PORT>              # Port to listen on (default: 3000)
--host <HOST>              # Host to bind to (default: 127.0.0.1)
--hive-url <URL>           # Hive server URL (required)
--wwb-token <TOKEN>        # Authentication token
```

---

## 🎯 Load Testing Scenarios

### **0. Test Dummy Server Setup**

```bash
# Default configuration (HTTP on 127.0.0.1:8080)
test-dummy

# Custom port and host for remote access
test-dummy --port 9000 --host 0.0.0.0

# For HTTPS testing (requires different HTTP server)
test-dummy --https --cert cert.pem --key key.pem

# Check available options
test-dummy --help
```

### **1. Basic Load Test**

```bash
# Start test dummy server on custom port
test-dummy --port 9000 --host 0.0.0.0

# Simple GET request test
curl -X PUT http://hiveaddress:4269/hive/poke \
  -H "Content-Type: application/json" \
  -d '{
    "target": "http://hiveaddress:9000/",
    "t": 10,
    "c": 100,
    "d": 60
  }'
```

### **2. POST Request with JSON Body**

```bash
# Test POST endpoint with JSON payload
curl -X PUT http://hiveaddress:4269/hive/poke \
  -H "Content-Type: application/json" \
  -d '{
    "target": "http://hiveaddress:8080/api/users",
    "method": "POST",
    "headers": {
      "Content-Type": "application/json",
      "Authorization": "Bearer your-token"
    },
    "body": "{\"name\":\"John Doe\",\"email\":\"john@example.com\"}",
    "t": 5,
    "c": 50,
    "d": 30
  }'
```

### **3. Custom Headers and Authentication**

```bash
# Test with custom headers and auth
curl -X PUT http://hiveaddress:4269/hive/poke \
  -H "Content-Type: application/json" \
  -d '{
    "target": "https://api.example.com/v1/data",
    "method": "GET",
    "headers": {
      "Authorization": "Bearer your-api-token",
      "X-API-Version": "2.0",
      "User-Agent": "WaspsWithBazookas/2.0"
    },
    "t": 20,
    "c": 200,
    "d": 120
  }'
```

### **4. Stress Testing**

```bash
# High-concurrency stress test
curl -X PUT http://hiveaddress:4269/hive/poke \
  -H "Content-Type: application/json" \
  -d '{
    "target": "http://hiveaddress:8080/api/heavy-operation",
    "t": 50,
    "c": 1000,
    "d": 300,
    "timeout": 10
  }'
```

---

## 📊 Monitoring and Results

### **Check Test Status**

```bash
# Get current status
curl http://hiveaddress:4269/hive/status

# Check if test is complete
curl http://hiveaddress:4269/hive/status/done

# Get full report
curl http://hiveaddress:4269/hive/status/report

# Get specific metric
curl http://hiveaddress:4269/hive/status/report/totalRPS
```

### **List Active Wasps**

```bash
# See all connected wasps
curl http://hiveaddress:4269/wasp/list

# Health check all wasps
curl http://hiveaddress:4269/wasp/boop/snoots
```

### **Sample Response**

```json
{
  "target": "http://hiveaddress:8080/api/users",
  "threads": 10,
  "concurrency": 100,
  "duration": 60,
  "totalRPS": 15420.5,
  "totalRequests": 925230,
  "latency": {
    "avg": 6.45,
    "max": 125.3
  },
  "status": {
    "completed": 3,
    "failed": 0
  },
  "wasp": {
    "reports": [
      {
        "wasp": {
          "id": "BuzzyBoi0",
          "ip": "127.0.0.1",
          "port": "3001"
        },
        "status": "complete",
        "stats": {
          "totalRPS": 5140.2,
          "totalRequests": 308412,
          "latency": {
            "avg": 6.45,
            "max": 125.3
          }
        }
      }
    ]
  }
}
```

---

## 🛠️ Advanced Features

### **1. Spawn Local Wasps**

```bash
# Spawn 5 local wasps automatically
curl http://hiveaddress:4269/hive/spawn/local/5
```


### **2. Ceasefire (Stop All Tests)**

```bash
# Stop all running tests
curl http://hiveaddress:4269/hive/ceasefire
```

### **3. Torch (Kill All Wasps)**

```bash
# Terminate all wasps
curl -X DELETE http://hiveaddress:4269/hive/torch
```

---

## 🌐 Distributed Testing

### **Multi-Server Setup**

```bash
# Server 1: Hive
hive --host 0.0.0.0 --port 4269 --wwb-token "secret123"

# Server 2: Wasp 1
wasp --hive-url http://server1:4269 --port 3001 --wwb-token "secret123"

# Server 3: Wasp 2
wasp --hive-url http://server1:4269 --port 3001 --wwb-token "secret123"

# Server 4: Wasp 3
wasp --hive-url http://server1:4269 --port 3001 --wwb-token "secret123"
```

### **Cloud Deployment**

```bash
# AWS EC2 Example
# Launch instances and configure security groups
# Then run wasps on each instance:

wasp --hive-url http://your-hive-ip:4269 \
     --host 0.0.0.0 \
     --port 3001 \
     --wwb-token "your-secret-token"
```

---

## 🔐 Security Best Practices

### **1. Use Authentication**

```bash
# Generate a secure token
openssl rand -hex 32

# Use it in both hive and wasps
hive --wwb-token "your-generated-token"
wasp --hive-url http://hiveaddress:4269 --wwb-token "your-generated-token"
```

### **2. Network Security**

```bash
# Configure firewall rules
sudo ufw allow 4269/tcp  # Hive port
sudo ufw allow 3000:3100/tcp  # Wasp ports

# Use VPN for distributed testing
# Configure security groups in cloud environments
```

### **3. Monitoring and Logging**

```bash
# Enable file logging
hive --log /var/log/waspswithbazookas/hive.log

# Monitor system resources
htop  # CPU/Memory usage
iotop # Disk I/O
nethogs # Network usage
```

---

## 🚨 Troubleshooting

### **Common Issues**

**Wasp won't connect to Hive**
```bash
# Check network connectivity
ping hive-server-ip
telnet hive-server-ip 4269

# Check firewall rules
sudo ufw status
```

**Test results are inconsistent**
```bash
# Check target server resources
htop
free -h
df -h

# Verify wrk installation
wrk --version
```

**High latency or timeouts**
```bash
# Check network latency
ping target-server
traceroute target-server

# Verify target server can handle load
# Consider reducing concurrency
```

### **Performance Tuning**

```bash
# Increase system limits
echo "* soft nofile 65536" >> /etc/security/limits.conf
echo "* hard nofile 65536" >> /etc/security/limits.conf

# Optimize network settings
echo "net.core.somaxconn = 65535" >> /etc/sysctl.conf
echo "net.ipv4.tcp_max_syn_backlog = 65535" >> /etc/sysctl.conf
sysctl -p
```

---


## 🎯 Best Practices

### **1. Start Small**
- Begin with low concurrency and gradually increase
- Monitor target server resources during tests
- Use realistic test durations (30-300 seconds)

### **2. Monitor Everything**
- Watch CPU, memory, and network usage
- Monitor target server logs for errors
- Check for bottlenecks in your infrastructure

### **3. Test Realistic Scenarios**
- Use production-like data and endpoints
- Test with realistic user patterns
- Include authentication and custom headers

### **4. Document Your Tests**
- Keep track of test configurations
- Document performance baselines
- Share results with your team

---

## 🎉 You're Ready!

You now have the knowledge to command your wasp army effectively! Remember:

- **Start small and scale up**
- **Monitor everything**
- **Use authentication in production**
- **Test responsibly**

**Happy load testing! 🐝🚀**

---

## 📚 Additional Resources

- **[API Documentation](API.md)** - Complete API reference
- **[Examples](https://github.com/Phara0h/WaspsWithBazookas/tree/main/examples)** - Real-world usage examples
- **[GitHub Issues](https://github.com/Phara0h/WaspsWithBazookas/issues)** - Report bugs and request features
- **[Discussions](https://github.com/Phara0h/WaspsWithBazookas/discussions)** - Community support and ideas
