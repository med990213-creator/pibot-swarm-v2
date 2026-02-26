<!-- 
  _________________________________________________________
 /                                                         \
|     🥧  PI-SWARM v8.0 - AI SECURITY SWARM               |
|                                                           |
|     Telegram-native AI agent for security operations      |
 \_________________________________________________________/
        \
         🐝  
-->

<h1 align="center">
  <br>
  <img src="https://raw.githubusercontent.com/Pi-Swarm/pibot-swarm-v2/main/assets/pi-logo.png" width="200" alt="Pi-Swarm Logo">
  <br>
  🥧 Pi-Swarm v8.0
  <br>
</h1>

<h4 align="center">AI-Powered Security Swarm Controlled via Telegram</h4>

<p align="center">
  <a href="https://github.com/Pi-Swarm/pibot-swarm-v2/releases">
    <img src="https://img.shields.io/badge/version-8.0-blue.svg?style=for-the-badge&colorA=21262d&colorB=58a6ff" alt="Version">
  </a>
  <a href="https://github.com/Pi-Swarm/pibot-swarm-v2/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/Pi-Swarm/pibot-swarm-v2/build.yml?style=for-the-badge&colorA=21262d&colorB=238636&label=BUILD" alt="Build Status">
  </a>
  <a href="https://github.com/Pi-Swarm/pibot-swarm-v2/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-green.svg?style=for-the-badge&colorA=21262d&colorB=8957e5" alt="License">
  </a>
  <a href="https://github.com/Pi-Swarm/pibot-swarm-v2/releases">
    <img src="https://img.shields.io/github/downloads/Pi-Swarm/pibot-swarm-v2/total?style=for-the-badge&colorA=21262d&colorB=f85149&label=DOWNLOADS" alt="Downloads">
  </a>
</p>

<p align="center">
  <a href="#-quick-start">Quick Start</a> •
  <a href="#-features">Features</a> •
  <a href="#-commands">Commands</a> •
  <a href="#-installation">Install</a> •
  <a href="#-architecture">Architecture</a>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/Pi-Swarm/pibot-swarm-v2/main/assets/demo.gif" width="600" alt="Demo">
</p>

---

## ✨ What's Pi-Swarm?

Pi-Swarm is a **distributed AI security agent** that lives in your Telegram. Think of it as your personal security analyst that you can summon anytime, anywhere.

> 🎯 **Mission**: Democratize security analysis through conversational AI

### 🌟 Key Highlights

| 🚀 **Simple** | 🤖 **AI-Powered** | 🔒 **Security-First** | 🌍 **Distributed** |
|:-------------:|:-----------------:|:---------------------:|:------------------:|
| One-line install | Local LLM support | No data leaves your machine | Swarm architecture |
| Telegram native | Multi-model support | Encrypted comms | Agent coordination |
| Zero config | Real-time analysis | Audit logging | Auto-scaling |

---

## 🚀 Quick Start

### One-Line Install

```bash
curl -fsSL https://raw.githubusercontent.com/Pi-Swarm/pibot-swarm-v2/main/install.sh | bash
```

That's it. Pi-Swarm will:
1. ⬇️ Download the binary (~4MB)
2. ⚙️ Install to your PATH
3. 🤖 Optionally set up local AI (Ollama)
4. 🔑 Configure your Telegram token

### Start & Chat

```bash
pi telegram
```

Then message your bot on Telegram!

---

## 🤖 Telegram Commands

Send these commands to your bot:

### 🎛️ Core Commands

| Command | Description | Example |
|---------|-------------|---------|
| `/status` | 📊 System status & health | `/status` |
| `/scan` | 🔍 Network vulnerability scan | `/scan 192.168.1.1` |
| `/audit` | 🔎 Audit code repository | `/audit https://github.com/user/repo` |

### 🧠 AI Commands

| Command | Description | Example |
|---------|-------------|---------|
| `/ask` | 💬 Ask AI anything | `/ask explain smart contracts` |
| `/search` | 🌐 Search web & analyze | `/search latest CVE 2025` |
| `/analyze` | 🧬 Deep security analysis | `/analyze contract.sol` |

### ⚙️ Utility

| Command | Description |
|---------|-------------|
| `/help` | 📖 Show all commands |
| `/ping` | 🏓 Check latency |

### 💬 Example Conversation

```
You:     /status
Pi-Swarm: 🥧 Pi-Swarm v8.0 Online
         ├─ Status:  ✅ Healthy
         ├─ AI Node: ✅ Connected (qwen2.5:1.5b)
         ├─ Uptime:   2h 34m
         └─ Tasks:    0 active / 42 completed

You:     /scan scanme.nmap.org
Pi-Swarm: 🔍 Scanning scanme.nmap.org...
         
         Results:
         ├─ Port 22:  SSH      🟢 Open
         ├─ Port 80:  HTTP     🟢 Open
         └─ Port 443: HTTPS    🟢 Open
         
         ⚠️  0 vulnerabilities found
         ✅ Scan complete! 3.2s

You:     /ask What is DeFi?
Pi-Swarm: 🧠 DeFi (Decentralized Finance) refers to financial 
         services built on blockchain technology that operate 
         without traditional intermediaries like banks...
```

---

## 📦 Installation Options

### Option 1: Automatic (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/Pi-Swarm/pibot-swarm-v2/main/install.sh | bash
```

### Option 2: Manual Download

| Platform | Architecture | Download | Size |
|----------|--------------|----------|------|
| Linux | AMD64 | [pi-linux-amd64](https://github.com/Pi-Swarm/pibot-swarm-v2/releases/download/v8.0/pi-linux-amd64) | ~4.2 MB |
| Linux | ARM64 | [pi-linux-arm64](https://github.com/Pi-Swarm/pibot-swarm-v2/releases/download/v8.0/pi-linux-arm64) | ~4.1 MB |
| macOS | AMD64 | [pi-macos-amd64](https://github.com/Pi-Swarm/pibot-swarm-v2/releases/download/v8.0/pi-macos-amd64) | ~4.3 MB |
| Windows | AMD64 | [pi-windows-amd64.exe](https://github.com/Pi-Swarm/pibot-swarm-v2/releases/download/v8.0/pi-windows-amd64.exe) | ~4.5 MB |

### Option 3: Build From Source

```bash
git clone https://github.com/Pi-Swarm/pibot-swarm-v2.git
cd pibot-swarm-v2
cargo build --release
```

---

## 🔑 Telegram Setup

### 1. Create Bot

Message [@BotFather](https://t.me/BotFather) on Telegram:

```
/newbot
> MyPiSwarm
> mypiswarm_bot
```

Copy your token: `123456789:ABCdefGHIjklMNOpqrsTUVwxyz`

### 2. Configure

```bash
pi config telegram.token "YOUR_TOKEN"
```

### 3. Run

```bash
pi telegram
```

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    🥧 PI-SWARM v8.0                           │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │  🕵️ Recon   │  │  🧠 Analyst │  │  📊 Planner │         │
│  │   Agent     │  │    Agent    │  │    Agent    │         │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘         │
│         │                │                │               │
│         └────────────────┼────────────────┘               │
│                          │                                  │
│                  ┌───────┴───────┐                        │
│                  │  🎯 Orchestrator│                        │
│                  └───────┬───────┘                        │
│                          │                                  │
│         ┌────────────────┼────────────────┐                 │
│         │                │                │               │
│  ┌──────┴──────┐  ┌──────┴──────┐  ┌──────┴──────┐        │
│  │   🤖 LLM    │  │ 📱 Telegram │  │   📝 Report │        │
│  │  Connector  │  │   Gateway   │  │   Generator │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
│                                                              │
└─────────────────────────────────────────────────────────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │   Local AI (Ollama) │
              │   ├─ qwen2.5:1.5b   │
              │   ├─ llama3.2:1b    │
              │   └─ Custom models  │
              └─────────────────────┘
```

---

## 🛡️ Security Features

- 🔐 **Zero external data sharing** - Everything runs locally
- 🔒 **Encrypted Telegram comms** - MTProto 2.0
- 📝 **Audit logging** - All actions logged locally
- 🏠 **Local-first AI** - Ollama runs on your machine
- ⚡ **No persistent cloud storage**

---

## 📊 Performance

| Metric | Value |
|--------|-------|
| Binary Size | ~4 MB |
| RAM Usage | ~50 MB base |
| Cold Start | <1 second |
| Response Time | <3 seconds (local AI) |
| Supported Platforms | Linux, macOS, Windows |

---

## 🔄 Updates

```bash
# Check for updates
pi update check

# Apply update
pi update

# Or reinstall with latest
curl -fsSL https://raw.githubusercontent.com/Pi-Swarm/pibot-swarm-v2/main/install.sh | bash
```

---

## 🛠️ Troubleshooting

<details>
<summary>❌ Bot not responding?</summary>

```bash
# Check if pi is running
pgrep -f "pi telegram"

# Check logs
pi logs --follow

# Verify token
pi config show telegram.token
```
</details>

<details>
<summary>🤖 AI not working?</summary>

```bash
# Check Ollama status
ollama list

# Pull model manually
ollama pull qwen2.5:1.5b

# Test AI directly
ollama run qwen2.5:1.5b "hello"
```
</details>

<details>
<summary>🔄 Connection issues?</summary>

```bash
# Test Telegram API
curl https://api.telegram.org/bot<YOUR_TOKEN>/getMe

# Restart Pi-Swarm
pi telegram --restart
```
</details>

---

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

```bash
# Clone
git clone https://github.com/Pi-Swarm/pibot-swarm-v2.git

# Create branch
git checkout -b feature/amazing-feature

# Commit
git commit -m "Add amazing feature"

# Push
git push origin feature/amazing-feature

# Open PR
```

---

## 📜 License

MIT License - see [LICENSE](LICENSE) file

---

## 🙏 Acknowledgments

- 🦀 Built with [Rust](https://www.rust-lang.org/)
- 🧠 AI powered by [Ollama](https://ollama.com)
- 🤖 Telegram via [teloxide](https://github.com/teloxide/teloxide)
- 🏗️ Architecture inspired by [Cline](https://github.com/cline/cline)

---

<p align="center">
  <br>
  <b>🥧 Simple. Fast. Powerful.</b>
  <br>
  <sub>Made with 💜 by Pi-Swarm</sub>
  <br><br>
  <a href="https://pi-swarm.github.io">Website</a> •
  <a href="https://github.com/Pi-Swarm">GitHub</a> •
  <a href="https://t.me/piswarm_bot">Demo Bot</a>
</p>

<!-- 
  🐝🐝🐝🐝🐝🐝🐝🐝🐝🐝🐝🐝🐝🐝🐝🐝
  The swarm is always watching. Stay secure!
  🐝🐝🐝🐝🐝🐝🐝🐝🐝🐝🐝🐝🐝🐝🐝🐝
-->
