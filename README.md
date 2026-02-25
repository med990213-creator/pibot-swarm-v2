<p align="center">
  <img src="https://github.com/Pi-Swarm/Pi-Swarm/raw/main/logo.png" width="120" alt="Pi Swarm Logo">
</p>

<h1 align="center">🛡️ Pi-Swarm Security Edition</h1>

<p align="center">
  <strong>An Enhanced OpenClaw Fork Specialized for Security Operations</strong>
</p>

---

## 🚀 What is Pi-Swarm?

**Pi-Swarm Security Edition** is an enhanced version of OpenClaw that integrates specialized security tools directly into the AI agent architecture.

While OpenClaw provides general-purpose AI assistance, Pi-Swarm adds:
- 🔍 **Code Security Auditing** (Rust, Solana, Python, JavaScript)
- 🌐 **Network Reconnaissance** (nmap integration)
- 🛠️ **Automated Patching** (AI-generated security fixes)
- 🧠 **Local-First AI** (Ollama/Qwen, no data leaves your machine)

## 📦 Installation

```bash
# 1. Clone the repository
git clone https://github.com/Pi-Swarm/pibot-swarm-v2.git
cd pibot-swarm-v2

# 2. Install Ollama (if not already installed)
curl -fsSL https://ollama.com/install.sh | sh
ollama pull qwen2.5:1.5b

# 3. Install scan tool (optional, for network features)
sudo apt install nmap  # Linux
brew install nmap      # macOS
```

## 🎮 Usage

### Quick Start

```bash
# Check system status (similar to 'openclaw status')
python3 pi.py status

# Security audit a file
python3 pi.py audit ./smart_contract.sol

# Audit a GitHub repository
python3 pi.py audit https://github.com/user/solana-project

# Network scan
python3 pi.py scan 192.168.1.1

# Autonomous security task
python3 pi.py task "Find reentrancy vulnerabilities in current directory"

# Direct AI conversation
python3 pi.py agent "What are common Solana vulnerabilities?"
```

### Detailed Examples

#### 1. Code Auditing
```bash
# Audit a Rust file (e.g., Solana program)
python3 pi.py audit ./src/lib.rs

# The AI will:
# - Read the code
# - Analyze for vulnerabilities
# - Report findings with specific line numbers
```

#### 2. Repository Auditing
```bash
# Clone and audit a GitHub repo
python3 pi.py audit https://github.com/KadeshX-Web3/KadeshX

# The AI will:
# - Clone the repository
# - Find source files
# - Analyze for security issues
```

#### 3. Network Scanning
```bash
# Scan a single host
python3 pi.py scan 192.168.1.1

# Scan a network range
python3 pi.py scan 192.168.1.0/24

# The AI will:
# - Run nmap scan
# - Analyze services
# - Report security risks
```

#### 4. Autonomous Tasks
```bash
# Complex multi-step security analysis
python3 pi.py task "Search for all Rust files, audit each for UncheckedAccount vulnerabilities, and generate a report"

# The AI will:
# - Plan the approach
# - Execute multiple tool calls
# - Synthesize findings
```

## 🏗️ Architecture

### How It Works

```
┌─────────────────────────────────────────┐
│         User Command (pi.py)            │
│  - status, audit, scan, task, agent   │
└──────────────────┬──────────────────────┘
                   │
┌──────────────────▼──────────────────────┐
│      Agent Runner (agent_runner.py)     │
│  Plans and coordinates tool execution   │
└──────────────────┬──────────────────────┘
                   │
┌──────────────────▼──────────────────────┐
│     Security Tools (tools.py)           │
│  • audit_repo    - Clone & audit repos  │
│  • scan_target   - Network scanning     │
│  • read_code     - Read source files    │
│  • write_patch   - Apply fixes          │
└──────────────────┬──────────────────────┘
                   │
┌──────────────────▼──────────────────────┐
│      AI Brain (provider.py)             │
│  Connects to Ollama (Qwen2.5:1.5b)     │
│  for intelligent analysis               │
└─────────────────────────────────────────┘
```

### Key Differences from Standard OpenClaw

| Feature | Standard OpenClaw | Pi-Swarm Security |
|---------|------------------|-------------------|
| General purpose | ✅ | ✅ |
| Security tools | ❌ | ✅ (Built-in) |
| Code auditing | ❌ | ✅ |
| Network scanning | ❌ | ✅ (nmap) |
| Auto-patching | ❌ | ✅ |
| Local AI only | Optional | Required (Ollama) |

## 📚 Documentation

- **[SECURITY.md](SECURITY.md)** - Complete security usage guide (Arabic & English)
- **[ARCHITECTURE.md](docs/ARCHITECTURE.md)** - Technical architecture details
- **[EXAMPLES.md](docs/EXAMPLES.md)** - More usage examples

## 🔧 Tools Reference

### Security Tools

| Tool | Description | Usage |
|------|-------------|-------|
| `audit_repo` | Clone and analyze GitHub repositories | `pi.py audit <url>` |
| `scan_target` | Network port scanning | `pi.py scan <ip>` |
| `read_code` | Read and analyze source files | Internal use |
| `write_patch` | Write security fixes to files | Internal use |
| `ask_ollama` | AI reasoning and analysis | Internal use |

### Supported Languages

- 🦀 **Rust** (Solana, general security)
- 🐍 **Python** (web apps, scripts)
- 💻 **JavaScript/TypeScript** (Node.js, frontend)
- ⚡ **Solidity** (Ethereum smart contracts)
- And more...

## 🌐 Comparison with Other Tools

| Tool | Type | Local AI | Security Focus | Auto-Fix |
|------|------|----------|----------------|----------|
| OpenClaw | General | Optional | ❌ | ❌ |
| **Pi-Swarm** | **Security** | **Required** | **✅ Built-in** | **✅** |
| GitHub Copilot | Coding | ❌ | ❌ | ❌ |
| ChatGPT | General | ❌ | ❌ | ❌ |
| Mythril | Security | ✅ | ✅ (Solidity only) | ❌ |

## 🛡️ Security & Privacy

- **Local-First**: All AI processing happens via Ollama on your machine
- **No External APIs**: No data sent to OpenAI, Anthropic, or others
- **Private Audits**: Your code never leaves your system
- **Transparent**: Open source, you control everything

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Add your security tools to `pi_core/tools.py`
4. Submit a pull request

## 📄 License

MIT License - Free for personal and commercial use.

---

<p align="center">
  <em>Sovereign Security Intelligence. Built on OpenClaw foundation.</em>
</p>
