# 🛡️ Pi-Swarm (Rust Implementation)

**Sovereign AI Security Agent - OpenClaw Architecture in Rust**

## ⚡ Why Rust?

| Feature | Node.js (OpenClaw) | Rust (Pi-Swarm) |
|---------|-------------------|-----------------|
| Speed | Good | 10-100x faster |
| Memory Safety | Garbage collection | Zero-cost safety |
| Binary Size | ~50MB+ | ~5MB single file |
| Startup Time | Seconds | Milliseconds |
| Native Performance | Interpreted | Compiled |
| Deployment | npm + dependencies | Single binary |

## 🏗️ Architecture (OpenClaw-Compatible)

```
pi (Rust Binary)
├─ Gateway     - Local daemon (port 18789)
├─ Session     - State management  
├─ Agent       - AI task execution
├─ Provider    - Ollama integration
├─ Tools       - Security operations
└─ Telegram    - Channel integration
```

## Commands (Identical to OpenClaw)

```bash
# System
cargo run -- status           # Like: openclaw status
cargo run -- onboard          # Like: openclaw onboard
cargo run -- help             # Show help

# Gateway
cargo run -- gateway           # Like: openclaw gateway
cargo run -- gateway -p 9999   # Custom port

# Agent
cargo run -- agent "How do I fix SQL injection?"
cargo run -- agent "Audit ./smart_contract.sol"

# Security Tasks
cargo run -- task "Find all .rs files and check for UnsafeAccount"
cargo run -- security audit ./my_project
cargo run -- security scan 192.168.1.1

# Telegram (when configured)
cargo run -- telegram  # Start Telegram bot
```

## Building

### Development
```bash
cd ~/pibot-swarm-v2
cargo build

# Run with debug
cargo run -- status
```

### Release (Optimized)
```bash
# Build optimized binary
cargo build --release

# Result: target/release/pi (~5MB)
ls -lh target/release/pi

# Install globally
sudo cp target/release/pi /usr/local/bin/
```

## Module Structure

| File | Purpose | OpenClaw Equivalent |
|------|---------|---------------------|
| `main.rs` | CLI routing | `bin/openclaw.mjs` |
| `gateway.rs` | Daemon server | `src/gateway/` |
| `session.rs` | Session state | `src/session-manager.ts` |
| `agent.rs` | AI execution | `src/agent/` |
| `provider.rs` | AI provider | `src/provider/` |
| `tools.rs` | Tool calling | `src/tools/` |
| `telegram.rs` | Channel | `extensions/telegram/` |

## Key Differences from Node.js Version

1. **Speed**: Rust compiles to native code
2. **Safety**: No null pointer exceptions, no garbage collection pauses
3. **Concurrency**: Thousands of sessions without blocking
4. **Binary**: Single file deployment
5. **Memory**: Predictable usage, no memory leaks

## Runtime Requirements

- **Runtime**: None (Rust binary is self-contained)
- **OS**: Linux, macOS, Windows
- **Optional**: Ollama (for AI), nmap (for scanning)

## Configuration

```bash
# Environment variables
export PI_TELEGRAM_TOKEN="your_token"  # For Telegram
export PI_PORT="18789"                  # Gateway port
```

## Coming Soon

- [ ] WebSocket gateway
- [ ] Multiple AI provider support
- [ ] Docker container
- [ ] Install via `cargo install`

---

🛡️ **Built with Rust. Powered by Ollama.**
