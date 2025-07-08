# 🐝 Wasps With Bazookas 🚀

> *"It's like bees with machine guns, but way more power!"*

<div align="center">
  <img src="https://i.imgur.com/u6JbkCf.png" alt="WaspsWithBazookas Logo" width="350" height="350" />
  
  [![Crates.io](https://img.shields.io/crates/v/waspswithbazookas)](https://crates.io/crates/waspswithbazookas)
  [![License: GPL-2.0](https://img.shields.io/badge/License-GPL%202.0-green.svg)](https://opensource.org/licenses/GPL-2.0)
  [![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
</div>

---

## 🎯 What the Heck is This?

**WaspsWithBazookas** is your ultimate distributed load testing weapon. Think of it as having an army of angry wasps, each armed with rocket launchers, ready to absolutely *obliterate* your servers with HTTP requests.

But here's the kicker: **it's not just powerful—it's smart, scalable, and actually fun to use.**

### 🚨 **LEGAL DISCLAIMER** 
**⚠️ DO NOT USE THIS TO DDOS SERVERS YOU DON'T OWN!** 
This tool is for legitimate load testing of your own infrastructure. Using it against others' servers is illegal and we're not responsible if you end up in digital jail. You've been warned! ⚖️

---

## 🌟 Why WaspsWithBazookas?

### 🚀 **Distributed Power**
- **One Hive, Many Wasps**: Coordinate an army of load testing wasps from any location from a single command center
- **Cloud-Native**: Deploy wasps across multiple servers, regions, or cloud providers
- **Auto-Scaling**: Spawn wasps on-demand to match your testing needs

### ⚡ **Performance That Hurts**
- **Rust-Powered**: Blazing fast performance with memory safety.
- **Focused on Real Results**: Aimed to beat or match `wrk` performance so you know your test results are your servers' real performance and not a limitation of the tool.
- **Real-Time Metrics**: Get detailed insights into latency, throughput, and error rates

### 🎮 **Developer Experience**
- **Simple CLI**: Start testing with just a few commands
- **REST API**: Full programmatic control for CI/CD integration
- **Real-Time Monitoring**: Watch your wasps wreak havoc in real-time
- **Authentication**: Secure your wasp army with token-based auth

### 🔧 **Production Ready**
- **Health Checks**: Automatic wasp monitoring and recovery
- **Graceful Shutdown**: Clean termination without leaving orphaned processes
- **Comprehensive Logging**: Detailed logs for debugging and analysis
- **Error Handling**: Robust error recovery and reporting

---

## 🎪 Use Cases That Actually Matter

### 🏢 **Enterprise Load Testing**
- **Microservices Testing**: Test individual services under realistic load
- **API Performance**: Validate your REST APIs can handle production traffic
- **Infrastructure Stress Testing**: Find out the bottlenecks in your infrastructure and know the overhead of each part of your stack.
- **CDN Performance**: Test content delivery across multiple regions

### 🚀 **DevOps & SRE**
- **Capacity Planning**: Understand your infrastructure limits before they matter
- **Chaos Engineering**: Intentionally bring things to their knees to make them stronger
- **Performance Regression Testing**: Catch performance issues before they hit production
- **Load Balancer Testing**: Ensure your load balancers work under stress

### 🎯 **Development Teams**
- **Feature Testing**: Validate new features under load before deployment
- **Performance Benchmarks**: Establish baseline performance metrics
- **Integration Testing**: Test how your services behave under load together
- **Release Validation**: Ensure new releases don't break performance

---

## 🏆 What Makes This Special?

### 🐝 **The Hive Architecture**
```
    🏠 HIVE (Command Center)
         ↕️
    🐝🐝🐝🐝🐝🐝🐝🐝
    Wasp Army Spread Out Across the World (or not)
         ↕️
    🎯 TARGET SERVER
```

- **Hive**: Your command center that coordinates all wasps
- **Wasps**: Individual load testing agents that do the heavy lifting
- **Distributed**: Each wasp runs independently, maximizing throughput

### ⚡ **Performance Highlights**
- **Millions of RPS**: Scale to millions of requests per second
- **Sub-microsecond Latency**: Precise timing measurements
- **Real-time Reporting**: Get results as they happen

### 🛡️ **Enterprise Features**
- **Authentication**: Secure your wasp army
- **Health Monitoring**: Automatic wasp health checks
- **Graceful Scaling**: Add/remove wasps without downtime
- **Comprehensive Logging**: Full audit trail of all operations

---

## 🚀 Quick Start

Want to see this in action? Here's how to get started in under 2 minutes:

```bash
# 1. Install the tools
curl -fsSL https://raw.githubusercontent.com/Phara0h/WaspsWithBazookas/main/install.sh | bash

# 2. Start the test dummy server (your target)
test-dummy --port 8080 --host 127.0.0.1

# 3. Start the hive (command center)
hive --port 4269

```
* Go to http://127.0.0.1:4269 
* Click "Spawn Local Wasps" 
* Set the target to http://127.0.0.1:8080 
* Click Launch the attack!

**Boom!** You just launched a distributed load test with 2 wasps, each using 10 threads and 100 connections for 30 seconds. That's 2,000 concurrent connections hitting your target! 🎯

> 💡 **Pro Tip**: The test dummy server now provides ultra-fast, httpbin-style endpoints for every load testing scenario. See the [Test Dummy Guide](TEST_DUMMY.md) for details! Very useful for testing infrastructure sicne you don't want the http server to be the bottleneck.

---

## 📚 What's Next?

- **[Installation Guide](INSTALL.md)** - Get WaspsWithBazookas running on your system many options available
- **[Usage Guide](RUN.md)** - Learn how to command your wasp army
- **[Test Dummy Guide](TEST_DUMMY.md)** - Use the built-in test server for ultra-fast, httpbin-style load testing
- **[API Documentation](API.md)** - Full REST API reference
- **[Postman Collection](https://github.com/Phara0h/WaspsWithBazookas/blob/main/waspswithbazookas.postman_collection.json)** - Postman collection for the API
- **[GitHub Issues](https://github.com/Phara0h/WaspsWithBazookas/issues)** - Report bugs and request features
- **[Discussions](https://github.com/Phara0h/WaspsWithBazookas/discussions)** - Community support and ideas
---

## 🤝 Contributing

Found a bug? Want to add features? We'd love your help! Check out our [Contributing Guide](CONTRIBUTING.md) and join the swarm! 🐝

---

## 📄 License

This project is licensed under the GNU General Public License v2.0 - see the [LICENSE](../LICENSE) file for details.

---

## Changelog

{{doc1}}

