# 🐝 Wasps With Bazookas 🚀

> *"It's like bees with machine guns, but way more power!"*

<div align="center">
  <img src="https://i.imgur.com/u6JbkCf.png" alt="WaspsWithBazookas Logo" width="350" height="350" />
  
  [![Crates.io](https://img.shields.io/crates/v/wasps-with-bazookas)](https://crates.io/crates/wasps-with-bazookas)
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

> 💡 **Pro Tip**: The test dummy server now provides ultra-fast, httpbin-style endpoints for every load testing scenario. See the [Test Dummy Guide](docs/TEST_DUMMY.md) for details! Very useful for testing infrastructure sicne you don't want the http server to be the bottleneck.

---

## 📚 What's Next?

- **[Installation Guide](docs/INSTALL.md)** - Get WaspsWithBazookas running on your system many options available
- **[Usage Guide](docs/RUN.md)** - Learn how to command your wasp army
- **[Test Dummy Guide](docs/TEST_DUMMY.md)** - Use the built-in test server for ultra-fast, httpbin-style load testing
- **[API Documentation](docs/API.md)** - Full REST API reference
- **[Postman Collection](https://github.com/Phara0h/WaspsWithBazookas/blob/main/waspswithbazookas.postman_collection.json)** - Postman collection for the API
- **[GitHub Issues](https://github.com/Phara0h/WaspsWithBazookas/issues)** - Report bugs and request features
- **[Discussions](https://github.com/Phara0h/WaspsWithBazookas/discussions)** - Community support and ideas
---

## 🤝 Contributing

Found a bug? Want to add features? We'd love your help! Check out our [Contributing Guide](CONTRIBUTING.md) and join the swarm! 🐝

---

## 📄 License

This project is licensed under the GNU General Public License v2.0 - see the [LICENSE](LICENSE) file for details.

---
---

### Changelog

All notable changes to this project will be documented in this file. Dates are displayed in UTC.

#### [v2.0.2](https://github.com/Phara0h/WaspsWithBazookas/compare/v2.0.1...v2.0.2)

- Update Cargo.toml [`3288762`](https://github.com/Phara0h/WaspsWithBazookas/commit/328876217bf890f8f9c05c6254b7586ef464c20a)
- Update README.md [`80c0213`](https://github.com/Phara0h/WaspsWithBazookas/commit/80c02136b050ed2405a4ac9998fe6b236b6c7b38)
- Removed chocolatey don't want the hassle [`ce5eae5`](https://github.com/Phara0h/WaspsWithBazookas/commit/ce5eae52542e736a61e7a4bb35468e09d97f4f7b)
- Update chocolatey.yml [`9c10910`](https://github.com/Phara0h/WaspsWithBazookas/commit/9c1091054237b1698be8106332fe3994496f8713)
- Fix Chocolatey workflow triggers - add multiple trigger types and better version handling [`b1170ca`](https://github.com/Phara0h/WaspsWithBazookas/commit/b1170cac90eee9b53b9b0bc2c039e1d5c1a06998)
- Fix Chocolatey workflow to properly create packages in chocolatey-community repository [`91f1869`](https://github.com/Phara0h/WaspsWithBazookas/commit/91f1869ddb44b7d288a96b3e199eb33f4347c0c3)
- Remove redundant homebrew.yml workflow - using release.yml for Homebrew core submission [`b1b2ff6`](https://github.com/Phara0h/WaspsWithBazookas/commit/b1b2ff68bd05ae062f23f009c857fea6eb093b88)
- homebrew fixes [`8d79830`](https://github.com/Phara0h/WaspsWithBazookas/commit/8d798305e6a917e7708508f947628ec1c8514eb3)
- Fix Homebrew formula to build from source and clean up old ARM64 references [`18f9375`](https://github.com/Phara0h/WaspsWithBazookas/commit/18f9375cc4a053d6bfdafdeff4619054f26b3615)
- Update Homebrew workflow to use mislav/bump-homebrew-formula-action for core submission [`29f296a`](https://github.com/Phara0h/WaspsWithBazookas/commit/29f296a792cae96cf99e4ce076b31c3958e00c47)
- Fix Homebrew formula to build from source instead of using cargo install [`f84178d`](https://github.com/Phara0h/WaspsWithBazookas/commit/f84178d02b41def445037d01dc713848fe1b2dfe)
- Fix Homebrew formula update action - downgrade to v3 and add debugging [`7ca1c8d`](https://github.com/Phara0h/WaspsWithBazookas/commit/7ca1c8df96aea8df7a86fe9bac12fb06518dea7a)
- Update INTRO.md [`45c25b4`](https://github.com/Phara0h/WaspsWithBazookas/commit/45c25b405718786974f2a204a270edb7f556f0e5)

#### [v2.0.1](https://github.com/Phara0h/WaspsWithBazookas/compare/v2.0.0...v2.0.1)

> 8 July 2025

- chore: bump version to 2.0.1 (patch) [`10ee889`](https://github.com/Phara0h/WaspsWithBazookas/commit/10ee889468b9e29d617e4984375c505cf483bbe9)
- Fixed windows issue with set_tcp_nodelay [`58ac9f1`](https://github.com/Phara0h/WaspsWithBazookas/commit/58ac9f17c732d50a26474a9a68d6aa87eb74b56b)
- Remove ARM64 Linux target due to OpenSSL cross-compilation issues [`d8ffc33`](https://github.com/Phara0h/WaspsWithBazookas/commit/d8ffc338bf1f708c5bd8430cfd057fdf5b39336b)
- Update release.yml [`98fa4f7`](https://github.com/Phara0h/WaspsWithBazookas/commit/98fa4f7415748c1021b047cd343c9e962382652b)
- Update release.yml [`98fad23`](https://github.com/Phara0h/WaspsWithBazookas/commit/98fad2330da4e6f05fe2e333651bdfa614042c5c)
- Update release.yml [`79c745d`](https://github.com/Phara0h/WaspsWithBazookas/commit/79c745d596582f638a12f6a207e43cb0a1422354)
- Update release.yml [`4ab5ba5`](https://github.com/Phara0h/WaspsWithBazookas/commit/4ab5ba5161c08362948962e0fde36971ee12ea2d)
- updated docs and attempt to fix release action [`98a24b4`](https://github.com/Phara0h/WaspsWithBazookas/commit/98a24b4436890ebc5de09383808c947e74023c7c)
- Update release.yml [`606a295`](https://github.com/Phara0h/WaspsWithBazookas/commit/606a29540651f21e130980f02796e58e58ec6a1a)
- Update release.yml [`de332e9`](https://github.com/Phara0h/WaspsWithBazookas/commit/de332e963840e045fe62254fa4881af5f0ed7ad1)
- Update release.yml [`efa001c`](https://github.com/Phara0h/WaspsWithBazookas/commit/efa001c101a6d26a14542bf19ba5ccfa47cf8197)
- Update release.yml [`040bfec`](https://github.com/Phara0h/WaspsWithBazookas/commit/040bfec1f47e4b7884fdb33a7f31381a623a5d4a)
- Updated workflows [`c595a54`](https://github.com/Phara0h/WaspsWithBazookas/commit/c595a54fbbde72bc295ebe0e4cdcb948a3f5d9c3)
- Update release.yml [`3039cea`](https://github.com/Phara0h/WaspsWithBazookas/commit/3039cea1bf9a367d39e19e702bb9116dbd32549a)
- Fixed readme [`d7215e7`](https://github.com/Phara0h/WaspsWithBazookas/commit/d7215e7cc670f9a92e41546134bc82f5da5a0434)
- Fix autochange log [`2b33b55`](https://github.com/Phara0h/WaspsWithBazookas/commit/2b33b55df4079d7819b29c19671d15ddb9a3bd5e)

### [v2.0.0](https://github.com/Phara0h/WaspsWithBazookas/compare/v1.3.0...v2.0.0)

> 8 July 2025

- Bump handlebars from 4.7.6 to 4.7.7 [`#6`](https://github.com/Phara0h/WaspsWithBazookas/pull/6)
- Bump fastify-static from 2.5.0 to 4.2.4 [`#7`](https://github.com/Phara0h/WaspsWithBazookas/pull/7)
- Bump ajv from 6.10.0 to 6.12.6 [`#8`](https://github.com/Phara0h/WaspsWithBazookas/pull/8)
- Bump minimist from 1.2.5 to 1.2.6 [`#9`](https://github.com/Phara0h/WaspsWithBazookas/pull/9)
- Bump node-fetch from 2.6.1 to 2.6.7 [`#10`](https://github.com/Phara0h/WaspsWithBazookas/pull/10)
- Bump qs from 6.5.2 to 6.5.3 [`#11`](https://github.com/Phara0h/WaspsWithBazookas/pull/11)
- Bump find-my-way from 2.2.3 to 2.2.5 [`#5`](https://github.com/Phara0h/WaspsWithBazookas/pull/5)
- Bump node-fetch from 2.6.0 to 2.6.1 [`#4`](https://github.com/Phara0h/WaspsWithBazookas/pull/4)
- Bump fastify from 2.4.1 to 2.15.1 [`#3`](https://github.com/Phara0h/WaspsWithBazookas/pull/3)
- chore: bump version to 2.0.0 (major) [`efc07ae`](https://github.com/Phara0h/WaspsWithBazookas/commit/efc07ae513bd517fc3f64ef274ece5fbe7f034ac)
- Created test dummy service, docs, workflows, and much more [`d1830b5`](https://github.com/Phara0h/WaspsWithBazookas/commit/d1830b5fd3e62aa6c918b81c5a348aedeaa62a7f)
- Init commit for full rewrite in rust + no more wrk depend [`82b21d7`](https://github.com/Phara0h/WaspsWithBazookas/commit/82b21d7bd5789c94884178ac8385e093ddf67497)

#### [v1.3.0](https://github.com/Phara0h/WaspsWithBazookas/compare/v1.2.8...v1.3.0)

> 28 April 2020

- Added /hive/ceasefire, removed node_modules and fixed security modules. [`5c57f6d`](https://github.com/Phara0h/WaspsWithBazookas/commit/5c57f6d0af921dd08ee38bf8dab9628fe59d5697)
- Set theme jekyll-theme-slate [`f279cd7`](https://github.com/Phara0h/WaspsWithBazookas/commit/f279cd7d72a98ffc064b97a4b0c2626f2184cb04)

#### [v1.2.8](https://github.com/Phara0h/WaspsWithBazookas/compare/v1.2.7...v1.2.8)

> 26 September 2019

- Added landing page on / [`c0b2aaf`](https://github.com/Phara0h/WaspsWithBazookas/commit/c0b2aafd68fd0e50aacc7ac357db3e593b42449e)
- Update README.md [`b65b226`](https://github.com/Phara0h/WaspsWithBazookas/commit/b65b226b2c32be93150c263931c5efe1abcab1d3)

#### [v1.2.7](https://github.com/Phara0h/WaspsWithBazookas/compare/1.2.4...v1.2.7)

> 26 September 2019

- Added landing page on / [`3c93610`](https://github.com/Phara0h/WaspsWithBazookas/commit/3c93610143002c55a8d01ad44123b89ddd2e1e74)
- Added landing page on / [`08c9e89`](https://github.com/Phara0h/WaspsWithBazookas/commit/08c9e894617717f0c16528307520669938793021)
- Delete _config.yml [`832a1dc`](https://github.com/Phara0h/WaspsWithBazookas/commit/832a1dcc321e60f7924688a7fa895180638918a6)
- Set theme jekyll-theme-cayman [`74b84fe`](https://github.com/Phara0h/WaspsWithBazookas/commit/74b84fea99c34fb7d356b1ac8b08e8281295989c)
- Update package.json [`427207f`](https://github.com/Phara0h/WaspsWithBazookas/commit/427207f3dd6ac22dc29dde4018d0e77a800ac649)
- Update README.md [`df8071f`](https://github.com/Phara0h/WaspsWithBazookas/commit/df8071f18e9ae25f9030039b1a5d8bcc48a241b4)

#### [1.2.4](https://github.com/Phara0h/WaspsWithBazookas/compare/1.2.3...1.2.4)

> 2 July 2019

- Ceasefire, wasp auto retry and bug squashing [`9230011`](https://github.com/Phara0h/WaspsWithBazookas/commit/9230011b431c88231e67d71b23cbde447226616c)

#### [1.2.3](https://github.com/Phara0h/WaspsWithBazookas/compare/1.2.2...1.2.3)

> 1 July 2019

- Percentage complete and invaild targets [`ea71892`](https://github.com/Phara0h/WaspsWithBazookas/commit/ea71892bf7122fe99777d0d3562fe7c6131276a2)

#### [1.2.2](https://github.com/Phara0h/WaspsWithBazookas/compare/1.2.1...1.2.2)

> 1 July 2019

- Secured from command injections [`ee96f90`](https://github.com/Phara0h/WaspsWithBazookas/commit/ee96f906766e592e870d72e0d9f8bd832fc1a8ce)
- Update README.md [`e03ffeb`](https://github.com/Phara0h/WaspsWithBazookas/commit/e03ffebcfd626e42b98f3053bdb96d69bf5e1357)

#### [1.2.1](https://github.com/Phara0h/WaspsWithBazookas/compare/1.2.0...1.2.1)

> 19 June 2019

- Fixed a quick bug to catch errors on poke [`b2133e7`](https://github.com/Phara0h/WaspsWithBazookas/commit/b2133e72b51aea08678b885f583641150429a458)

#### [1.2.0](https://github.com/Phara0h/WaspsWithBazookas/compare/1.1.0...1.2.0)

> 19 June 2019

- Wrk params reported,  auto checkin in wasps & snoot booping [`614d8e4`](https://github.com/Phara0h/WaspsWithBazookas/commit/614d8e44873475e542eae8813f6a3ec6ddf7130a)
- Create SECURITY.md [`4416182`](https://github.com/Phara0h/WaspsWithBazookas/commit/4416182199ac3152f567bdaa7eb4906130362092)

#### 1.1.0

> 19 June 2019

- Auto wasp regeneration, error logging and misc bug fixes [`5ff11e6`](https://github.com/Phara0h/WaspsWithBazookas/commit/5ff11e60bf5989f90799f6d5ae75268661062338)
- Added wasp heartbeats, local wasp spawning from hive and wrk timeout option [`2bebc1d`](https://github.com/Phara0h/WaspsWithBazookas/commit/2bebc1d824db0569522dd32974f152ee43ed0f4b)
- update package num [`bbd2bed`](https://github.com/Phara0h/WaspsWithBazookas/commit/bbd2bed509c1fecc25d86e92b3e05cc04e0a7528)
- added file logging [`f8b726d`](https://github.com/Phara0h/WaspsWithBazookas/commit/f8b726d0df6a5aed09f96f968dcf3cbe5498157e)
- fixed cli poke bug and hive poke parse bug [`e7af2b5`](https://github.com/Phara0h/WaspsWithBazookas/commit/e7af2b5ac2cf3d2f38f2f0aae323e06600804586)
- Fixed various bugs and spelling errors [`4797516`](https://github.com/Phara0h/WaspsWithBazookas/commit/479751642ba95feb4ce806e25ff348841af64e37)
- added * ip listen [`ebff346`](https://github.com/Phara0h/WaspsWithBazookas/commit/ebff3466faacbadbf8aa0b96e198e03e8572c168)
- Update package.json [`61e6179`](https://github.com/Phara0h/WaspsWithBazookas/commit/61e61790bb4fca46e9fb7570d27e47929e847a74)
- Update wwb-cli-spawn-local.js [`d4d0f03`](https://github.com/Phara0h/WaspsWithBazookas/commit/d4d0f03043da4623d56b2d9bb1c722bc48dc3dde)
- Fixed some bugs wrk lua scripts should work now. [`936551e`](https://github.com/Phara0h/WaspsWithBazookas/commit/936551e3c1544c82160ccf95aaec4751d066a213)
- Update README.md [`75c577f`](https://github.com/Phara0h/WaspsWithBazookas/commit/75c577f22a8acc4632c5e22d30935173f8e9792e)
- Update README.md [`0a1c634`](https://github.com/Phara0h/WaspsWithBazookas/commit/0a1c63433eca7bf738fe934b41ef65dc9c908a84)
- Basic command line functionality [`7a0cf7e`](https://github.com/Phara0h/WaspsWithBazookas/commit/7a0cf7e883c98c236aad1364404ac34eb40e073b)
- Added test server [`a347e1f`](https://github.com/Phara0h/WaspsWithBazookas/commit/a347e1f15c1da268b5a7497c2612467d14860073)
- Fixed nonSuccessRequests [`1a06650`](https://github.com/Phara0h/WaspsWithBazookas/commit/1a06650e142e10015ba6931deb7fc94e19f49e16)
- Update README.md [`de9ad82`](https://github.com/Phara0h/WaspsWithBazookas/commit/de9ad82beb5ea592b9ae851c76b15b7ac2793418)
- Set theme jekyll-theme-minimal [`f805dc8`](https://github.com/Phara0h/WaspsWithBazookas/commit/f805dc80590853f54a547b8129570eaa71cbb852)
- Set theme jekyll-theme-slate [`249f804`](https://github.com/Phara0h/WaspsWithBazookas/commit/249f8043e209677c755fdc4680b59ca6cffc4d25)
- Update API.md [`7536fa3`](https://github.com/Phara0h/WaspsWithBazookas/commit/7536fa321c6ac96ef131e7d8eb49470c58f695d5)
- Update README.md [`5ca7f2e`](https://github.com/Phara0h/WaspsWithBazookas/commit/5ca7f2eacdf7e05c57006caaf801bcf31f093fa8)
- Added some init docs [`3b91133`](https://github.com/Phara0h/WaspsWithBazookas/commit/3b91133104524cb928b20272b0652fc4634e4a9b)
- WRK scripts now work, reporting and the rest of the functionality [`dace65c`](https://github.com/Phara0h/WaspsWithBazookas/commit/dace65c8d25d899f9968e1346f877c60e33df472)
- Init commit basic functionality working [`9ca05ba`](https://github.com/Phara0h/WaspsWithBazookas/commit/9ca05babd22624f625ec80569dfe330166e31e43)
- Initial commit [`efffbf3`](https://github.com/Phara0h/WaspsWithBazookas/commit/efffbf37fea5ba0e9716517af73cbc11ac95cbd9)
