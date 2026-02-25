<p align="center">
  <img src="https://github.com/Pi-Swarm/Pi-Swarm/raw/main/logo.png" width="120" alt="Pi Swarm Logo">
</p>

<h1 align="center">🛡️ PI SWARM SOVEREIGN CORE (v3.4)</h1>

<p align="center">
  <strong>The Advanced Security Swarm Engine. Built on OpenClaw Philosophy.</strong>
</p>

---

## 🚀 How to Run the Swarm (Quick Start)

### 1. Prerequisites
- **Python 3.10+**
- **Ollama** installed: `curl -fsSL https://ollama.com/install.sh | sh`
- **Qwen2.5 Model:** `ollama run qwen2.5:1.5b`

### 2. Installation
```bash
git clone https://github.com/Pi-Swarm/pibot-swarm-v2.git
cd pibot-swarm-v2
python3 -m venv venv && source venv/bin/activate
```

### 3. Execution (The OpenClaw Way)
Run the core orchestrator using the unified CLI:

- **Check System Health:**
  ```bash
  python3 pi.py status
  ```

- **Run an Autonomous Security Task:**
  ```bash
  python3 pi.py task "Scan repository https://github.com/user/repo for vulnerabilities"
  ```

---

## 🧠 System Architecture (Pi-Claw Core)
The system is divided into functional layers mimicking the **OpenClaw** architecture:
- **pi_core/provider.py**: Manages secure connectivity to the local LLM brain.
- **pi_core/agent_runner.py**: Executes the Thought-Action-Result reasoning loop.
- **pi_core/tools.py**: The specialized arsenal of security and recon tools.
- **pi.py**: The central Command Line Interface (CLI).

---
## 📡 Stay Connected
- **Official Hub:** [Pi-Swarm.github.io](https://Pi-Swarm.github.io)
- **Methodology:** [Sovereign Audit Standard v1.1](docs/intelligence/AUDIT_METHODOLOGY.md)

<p align="center">
  <em>Sovereign Intelligence. No Centralized Dependency. Built to Secure.</em>
</p>
