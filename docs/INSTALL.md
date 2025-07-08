# 🛠️ Installation Guide

> *"Getting your wasp army ready for deployment!"*

---

## 🎯 Prerequisites

Before you can unleash the wasps, you'll need a few things:

### 🔧 **System Requirements**
- **Rust 1.70+**: For manually building the wasps and hive
- **Network Access**: For distributed testing across multiple machines

### 💻 **Supported Platforms**
- ✅ **Linux** (Ubuntu, CentOS, RHEL, etc.)
- ✅ **macOS** (10.15+)
- ✅ **Windows** (WSL2 recommended)

---

## 🚀 Quick Installation

### **Option 1: Universal Installer (Recommended)**

**Linux/macOS:**
```bash
# One-line installation
curl -fsSL https://raw.githubusercontent.com/Phara0h/WaspsWithBazookas/main/install.sh | bash

# Or download and run manually
wget https://raw.githubusercontent.com/Phara0h/WaspsWithBazookas/main/install.sh
chmod +x install.sh
./install.sh
```

**Windows (PowerShell):**
```powershell
# One-line installation
Invoke-Expression (Invoke-WebRequest -Uri "https://raw.githubusercontent.com/Phara0h/WaspsWithBazookas/main/install.ps1").Content

# Or download and run manually
Invoke-WebRequest -Uri "https://raw.githubusercontent.com/Phara0h/WaspsWithBazookas/main/install.ps1" -OutFile "install.ps1"
.\install.ps1
```

### **Option 2: Package Managers**

**macOS (Homebrew):**
```bash
brew install waspswithbazookas
```

**Windows (Chocolatey):**
```powershell
# Install
choco install waspswithbazookas
```

### **Option 3: Docker**

```bash
# Pull and run with Docker
docker run -p 4269:4269 -p 8080:8080 phara0h/waspswithbazookas:latest

# Or use Docker Compose for full setup
git clone https://github.com/Phara0h/WaspsWithBazookas.git
cd WaspsWithBazookas
docker-compose up -d
```

### **Option 4: Manual Download**

1. Go to [GitHub Releases](https://github.com/Phara0h/WaspsWithBazookas/releases)
2. Download the appropriate binary for your system
3. Extract and add to your PATH

### **Option 5: Cargo Install (Requires Rust)**

```bash
# Install from crates.io (when available)
cargo install waspswithbazookas

# Or install from source
cargo install --git https://github.com/Phara0h/WaspsWithBazookas.git
```

### **Option 6: Build from Source**

```bash
# Clone the repository
git clone https://github.com/Phara0h/WaspsWithBazookas.git
cd WaspsWithBazookas

# Build the project
cargo build --release

# Install the binaries
cargo install --path .
```

---

## 🔧 Installing Dependencies

### **1. Install Rust**

If you don't have Rust installed:

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload your shell
source ~/.bashrc  # or source ~/.zshrc

# Verify installation
rustc --version
cargo --version
```

### **2. Verify Installation**

```bash
# Check that everything is installed
hive --version
wasp --version
test-dummy --version

# You should see something like:
# hive 2.0.0
# wasp 2.0.0
# test-dummy 2.0.0
```

---

## 🔐 Security Setup (Optional but Recommended)

### **Generate Authentication Token**

```bash
# Generate a secure token
openssl rand -hex 32

# This will output something like:
# a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456
```

### **Configure Firewall Rules**

```bash
# Allow hive port (default: 4269)
sudo ufw allow 4269

# Allow wasp ports (default: 3000-3100)
sudo ufw allow 3000:3100/tcp

# For cloud deployments, configure security groups accordingly
```

---

## 🧪 Test Your Installation

Let's make sure everything works:

```bash
# 1. Start the test dummy server (optional)
test-dummy --port 8080 --host 127.0.0.1

# 2. Start the hive
hive --port 4269

# 3. In another terminal, start a wasp
wasp --hive-url http://localhost:4269 --port 3001

# 4. Test the connection
curl http://localhost:4269/wasp/list

# You should see your wasp in the list!
```

---

## 🚨 Troubleshooting

### **Common Issues**

**"Permission denied"**
```bash
# Make sure you have execute permissions
chmod +x $(which hive)
chmod +x $(which wasp)
```

**"Connection refused"**
```bash
# Check if ports are available
netstat -tulpn | grep :4269
netstat -tulpn | grep :3001

# Kill any processes using those ports
sudo kill -9 <PID>
```

**"Rust not found"**
```bash
# Reinstall Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.bashrc
```